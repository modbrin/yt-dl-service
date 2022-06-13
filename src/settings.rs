use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

static SETTINGS_PATH: &'static str = "settings.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEntity {
    pub url: String,
    pub output_path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub tasks: Vec<DownloadEntity>,
    pub update_time: String,
    pub log_path: Option<String>,
}

pub fn load_settings() -> Result<Settings, &'static str> {
    let settings_file = File::open(SETTINGS_PATH).map_err(|_| "Failed to locate settings file.")?;
    let reader = BufReader::new(settings_file);
    let settings = serde_json::from_reader(reader)
        .map_err(|_| "Failed to deserialize settings file contents.")?;
    Ok(settings)
}
