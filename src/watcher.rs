use crate::settings::{DownloadEntity, Settings};
use macros::return_on_err;
use std::cell::Cell;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{debug, error, trace};

type VideoId = String;

/// Temp files are downloaded to tmp directory located inside given output dir.
static TEMP_DIR: &str = "tmp";

pub fn contains_unfinished_downloads<P>(dir_path: P) -> Result<bool, &'static str>
where
    P: AsRef<Path>,
    PathBuf: From<P>,
{
    let mut tmp_path: PathBuf = PathBuf::from(dir_path);
    tmp_path.push(TEMP_DIR);
    let paths = read_dir(tmp_path).map_err(|_| "Failed to list directory via indicated path.")?;
    let tmp_count = paths.into_iter().count();
    Ok(tmp_count > 0)
}

// TODO: implement
pub fn get_unfinished_downloads<P>(dir_path: P) -> Result<HashSet<VideoId>, &'static str>
where
    P: AsRef<Path>,
    PathBuf: From<P>,
{
    todo!()
}

// TODO: implement
pub fn remove_tmp_if_empty() -> Result<(), &'static str> {
    todo!()
}

/// Process single provided task.
pub async fn process_task(task: &DownloadEntity) -> Result<(), &'static str> {
    let (send, recv) = oneshot::channel::<()>();
    let mut child = Command::new("yt-dlp")
        .stdout(Stdio::piped())
        .arg("-P")
        .arg(&task.output_path)
        .arg("-P")
        .arg(format!("temp:{}", TEMP_DIR))
        .arg(&task.url)
        .spawn()
        .map_err(|_| "Failed to spawn command.")?;
    let stdout = child
        .stdout
        .take()
        .ok_or("Failed to get stdout handle from child.")?;
    let mut reader = BufReader::new(stdout).lines();

    tokio::select! {
        retcode = child.wait() => {
            trace!("yt-dlp subprocess exited with: {:?}", retcode);
        },
        _ = async move {
            let send_cell = Cell::new(Option::Some(send));
            while let Ok(Some(line)) = reader.next_line().await {
                trace!("{}", line);
                if line.trim_end().ends_with("has already been downloaded") {
                    debug!("Found video that has already been downloaded, stopping.");
                    let _ = match send_cell.take() {
                        Some(s) => s.send(()),
                        _ => Ok(()),
                    };
                }
            }
        } => {},
        _ = recv => child.kill()
                       .await
                       .map_err(|_| "Failed to stop yt-dlp subprocess.")?
    }
    Ok(())
}

/// Iterate over available tasks and process each one linearly.
pub async fn process_all(tasks: &[DownloadEntity]) {
    // TODO: possibility for concurrent downloads?
    for task in tasks.iter() {
        if let Err(e) = process_task(task).await {
            error!("Task reported error: {}", e);
        }
    }
}

pub mod macros {
    /// If error occurs, log the error along with provided message and return.
    macro_rules! return_on_err {
        ($try:expr, $msg:literal) => {
            match $try {
                Ok(v) => v,
                Err(e) => {
                    error!("{}: {}", $msg, e);
                    return;
                }
            }
        };
    }

    pub(crate) use return_on_err;
}

pub struct Watcher {
    settings: Settings,
}

impl Watcher {
    pub fn new(settings: Settings) -> Self {
        Watcher { settings }
    }

    /// Executes indefinitely, processing tasks according to schedule.
    pub async fn run(&self) {
        let (send, mut recv) = mpsc::channel::<()>(1);
        let scheduler = return_on_err!(JobScheduler::new(), "Failed to create scheduler");

        let job = return_on_err!(
            Job::new_async(
                self.settings.update_schedule.as_str(),
                move |_uuid, _lock| {
                    let send = send.clone();
                    Box::pin(async move {
                        debug!("Scheduler ping");
                        let res = send.send(()).await;
                        if res.is_err() {
                            error!("Can't send scheduler ping: {}", res.unwrap_err());
                        }
                    })
                }
            ),
            "Failed to create job"
        );

        return_on_err!(scheduler.add(job), "Failed to add a job to scheduler");
        return_on_err!(scheduler.start(), "Failed to start scheduler");
        loop {
            // block current thread until scheduler ping
            if (recv.recv().await).is_some() {
                process_all(&self.settings.tasks).await;
            }
        }
    }
}
