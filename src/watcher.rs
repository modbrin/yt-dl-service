use std::cell::Cell;
use std::collections::BTreeSet;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::oneshot;
use tracing::{debug, trace};

use crate::settings::DownloadEntity;

type VideoId = String;

/// Temp files are downloaded to tmp directory located inside given output dir.
static TEMP_DIR: &'static str = "tmp";

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
pub fn get_unfinished_downloads<P>(dir_path: P) -> Result<BTreeSet<VideoId>, &'static str>
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

pub fn setup_watcher() -> Result<(), &'static str> {
    Ok(())
}
