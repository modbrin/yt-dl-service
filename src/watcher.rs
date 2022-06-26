use crate::settings::{parse_time, DownloadEntity, ScheduledTime, Settings};
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
use tracing::{debug, error, trace, warn};

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
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--no-progress",
            "-P",
            &task.output_path,
            "-P",
            format!("temp:{}", TEMP_DIR).as_str(),
        ])
        .args(&task.get_extra_flags())
        .arg(&task.url)
        .spawn()
        .map_err(|_| "Failed to spawn yt-dlp subprocess.")?;
    let stdout = child
        .stdout
        .take()
        .ok_or("Failed to get stdout handle from yt-dlp subprocess.")?;
    let stderr = child
        .stderr
        .take()
        .ok_or("Failed to get stderr handle from yt-dlp subprocess.")?;
    let mut reader_stdout = BufReader::new(stdout).lines();
    let mut reader_stderr = BufReader::new(stderr).lines();

    tokio::select! {
        retcode = child.wait() => {
            trace!("yt-dlp subprocess exited with: {:?}", retcode);
        },
        _ = async move {
            let send_cell = Cell::new(Option::Some(send));
            while let Ok(Some(line)) = reader_stdout.next_line().await {
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
        _ = async move {
            while let Ok(Some(line)) = reader_stderr.next_line().await {
                warn!("{}", line);
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

pub struct Watcher {
    settings: Settings,
}

impl Watcher {
    pub fn new(settings: Settings) -> Self {
        Watcher { settings }
    }

    /// Executes indefinitely, processing tasks according to schedule.
    pub async fn run(&self) -> Result<(), &'static str> {
        let (send, mut recv) = mpsc::channel::<()>(1);
        let scheduler = JobScheduler::new().map_err(|_| "Failed to create scheduler")?;

        if let Some(update_now) = self.settings.update_on_start {
            if update_now {
                let res = send.send(()).await;
                if res.is_err() {
                    error!("Can't send scheduler ping: {}", res.unwrap_err());
                }
            }
        }

        for sched in self.settings.update_schedule.iter() {
            let send = send.clone();
            let cron_str = match sched {
                ScheduledTime::Cron(c) => c.clone(),
                ScheduledTime::Daily(t) => {
                    // TODO: this code is in dire need of pre-processed settings
                    if let Some((h, m)) = parse_time(t) {
                        format!("0 {} {} * * *", m, h)
                    } else {
                        error!("Unable to parse time \"{}\", skipping", t);
                        continue;
                    }
                }
            };
            let job = Job::new_async(cron_str.as_str(), move |_uuid, _lock| {
                let send = send.clone();
                Box::pin(async move {
                    debug!("Scheduler ping");
                    let res = send.send(()).await;
                    if res.is_err() {
                        error!("Can't send scheduler ping: {}", res.unwrap_err());
                    }
                })
            })
            .map_err(|_| "Failed to create async job")?;
            scheduler
                .add(job)
                .map_err(|_| "Failed to add a job to scheduler")?;
        }

        scheduler.start().map_err(|_| "Failed to start scheduler")?;
        loop {
            // block current thread until scheduler ping
            if (recv.recv().await).is_some() {
                process_all(&self.settings.tasks).await;
            }
        }
    }
}
