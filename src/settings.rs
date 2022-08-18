use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};
use tracing::debug;

static DEFAULT_AUDIO_FORMAT: &str = "mp3";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEntity {
    /// Url for video or channel, any resource which yt-dlp supports
    pub url: String,
    /// Path where downloaded videos will be stored
    pub output_path: String,
    /// Human readable tag for this resource
    pub name: String,
    /// Extract audio, if `audio_format` is not set then default is mp3
    pub audio_only: Option<bool>,
    /// Has effect when `audio_only=true`, define output audio file format,
    /// possible options: mp3, aac, m4a, opus, vorbis, flac, alac, wav
    pub audio_format: Option<String>,
    /// Limit max resolution (i.e. video height), has no effect when `audio_only=true`
    /// examples: 480, 720, 1080, 1440, 2160, 4320
    pub max_resolution: Option<u32>,
    /// Limit download speed, used to prevent hogging a network connection
    /// examples: 3.5M, 300K
    pub throttle_speed: Option<String>,
    /// Custom flags to pass for yt-dlp, correctness is not checked
    pub custom_flags: Option<Vec<String>>,
}

impl DownloadEntity {
    pub fn get_extra_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        if let Some(is_audio_only) = self.audio_only {
            if is_audio_only {
                let format = self
                    .audio_format
                    .as_ref()
                    .map(String::as_str)
                    .unwrap_or(DEFAULT_AUDIO_FORMAT);
                flags.extend(
                    ["-f", "ba", "-x", "--audio-format", format]
                        .iter()
                        .map(|&v| v.to_string()),
                );
            }
        } else if let Some(max_height) = self.max_resolution {
            flags.extend(
                [
                    "-f".to_string(),
                    format!("bestvideo[height<={}]+bestaudio", max_height),
                ]
                .into_iter(),
            );
        }
        if let Some(speed_limit) = self.throttle_speed.clone() {
            flags.extend(["-r".to_string(), speed_limit].into_iter());
        }
        if let Some(custom) = self.custom_flags.clone() {
            flags.extend(custom.into_iter());
        }
        debug!("extra flags: {:?}", flags);
        flags
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Videos/Channels to download
    pub tasks: Vec<DownloadEntity>,
    /// Schedule for running tasks in cron format
    pub update_schedule: Vec<ScheduledTime>,
    /// Custom log path
    pub log_dir: Option<String>,
    /// Force run tasks on start of program, regardless of schedule
    pub update_on_start: Option<bool>,
    /// Log level: Error, Warn, Info, Debug, Trace
    pub log_level: Option<LogLevel>,
    /// Set file owner for downloaded files
    pub set_owner: Option<String>,
    /// Set file group for downloaded files
    pub set_group: Option<String>,
}

/// Parse settings from json file, panics on error
pub fn load_settings(path: &str) -> Settings {
    let settings_file = File::open(path)
        .map_err(|e| eprintln!("Failed to locate settings file: {}", e))
        .unwrap();
    let reader = BufReader::new(settings_file);
    let settings = serde_json::from_reader(reader)
        .map_err(|e| eprintln!("Failed to deserialize settings file contents: {}", e))
        .unwrap();
    settings
}

/// Perform preliminary checks to ensure that settings values are well-formed
pub fn validate_settings(settings: &Settings) -> Result<(), &'static str> {
    if settings.tasks.is_empty() {
        return Err("Tasks list is empty.");
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ScheduledTime {
    /// Execute single time, at specified time in format `hour:minute`
    Daily(String),
    /// Custom execution rule in cron format
    Cron(String),
}

/// Parse string in format `hour:minute` and return (hour, minute),
/// return None if parsing has failed
pub fn parse_time(value: &str) -> Option<(u8, u8)> {
    let nums: Vec<_> = value.split(":").collect();
    if nums.len() != 2 {
        return None;
    }
    let hour = nums[0].parse().ok()?;
    let minute = nums[1].parse().ok()?;
    Some((hour, minute))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for tracing::metadata::LevelFilter {
    fn from(level: LogLevel) -> Self {
        use LogLevel::*;
        match level {
            Off => Self::OFF,
            Error => Self::ERROR,
            Warn => Self::WARN,
            Info => Self::INFO,
            Debug => Self::DEBUG,
            Trace => Self::TRACE,
        }
    }
}
