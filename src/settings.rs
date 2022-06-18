use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

static SETTINGS_PATH: &'static str = "settings.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEntity {
    /// Url for video or channel, any resource which yt-dlp supports
    pub url: String,
    /// Path where downloaded videos will be stored
    pub output_path: String,
    /// Human readable tag for this resource
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Videos/Channels to download
    pub tasks: Vec<DownloadEntity>,
    /// Schedule for running tasks in cron format
    pub update_schedule: String,
    /// Custom log path
    pub log_path: Option<String>,
    /// Force run tasks on start of program, regardless of schedule
    pub update_on_start: Option<bool>,
}

pub fn load_settings() -> Result<Settings, &'static str> {
    let settings_file = File::open(SETTINGS_PATH).map_err(|_| "Failed to locate settings file.")?;
    let reader = BufReader::new(settings_file);
    let settings = serde_json::from_reader(reader)
        .map_err(|_| "Failed to deserialize settings file contents.")?;
    Ok(settings)
}
