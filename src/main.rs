use std::error::Error;
use tracing::{debug, info};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use watcher::Watcher;

mod settings;
mod watcher;

static DEFAULT_LOG_PATH: &'static str = "./";
static LOG_NAME: &'static str = "yt-dl-service.log";

fn setup_logger(log_dir: &str, log_name: &str) -> impl Drop {
    let (file_nb, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never(log_dir, log_name));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(file_nb);
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(EnvFilter::from_default_env())
        .init();
    guard
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let s = settings::load_settings().expect("Failed to get settings.");
    let log_path = s
        .log_path
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_LOG_PATH);
    let _guard = setup_logger(log_path, LOG_NAME);
    info!("Startup ok");
    debug!("{:?}", s);
    let _ = dbg!(watcher::contains_unfinished_downloads("./test"));
    // let _ = dbg!(watcher::process_task(&s.tasks[0]).await);
    // let _ = dbg!(watcher::process_task(&s.tasks[0]).await);

    Watcher::new(s).run().await;

    Ok(())
}
