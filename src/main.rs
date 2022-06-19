use settings::validate_settings;
use settings::LogLevel;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info};
use tracing_subscriber::prelude::*;
use watcher::Watcher;

mod settings;
mod watcher;

static DEFAULT_LOG_PATH: &str = "./";
static LOG_NAME: &str = "yt-dl-service.log";

fn setup_logger(log_dir: &str, log_name: &str, level: LogLevel) -> impl Drop {
    let (file_nb, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never(log_dir, log_name));
    let level: LevelFilter = level.into();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(file_nb)
        .with_filter(level);
    tracing_subscriber::registry().with(fmt_layer).init();
    guard
}

#[tokio::main]
async fn main() {
    let s = match settings::load_settings() {
        Ok(v) => v,
        Err(e) => {
            error!("Stopping, failed to get settings: {}", e);
            return;
        }
    };
    let log_path = s.log_path.as_deref().unwrap_or(DEFAULT_LOG_PATH);
    let log_level = s.log_level.clone().unwrap_or(LogLevel::Info);
    let _guard = setup_logger(log_path, LOG_NAME, log_level);
    debug!("{:?}", s);
    if let Err(e) = validate_settings(&s) {
        error!("Stopping, failed to validate settings: {}", e);
        return;
    }
    info!("Startup OK.");

    if let Err(e) = Watcher::new(s).run().await {
        error!("Stopping, runtime error: {}", e);
    }
}
