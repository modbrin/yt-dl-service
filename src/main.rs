use tracing::instrument::WithSubscriber;
use tracing::info;

mod watcher;
mod settings;

static DEFAULT_LOG_PATH: &'static str = "./";
static LOG_NAME: &'static str = "common.log"; 

fn setup_logger(log_dir: &str, log_name: &str) -> impl Drop {
    let (file_nb, guard) = tracing_appender::non_blocking(
        tracing_appender::rolling::never(log_dir, log_name)
    );
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(file_nb)
        .init();
    guard
}

fn main() {
    let s = settings::load_settings().expect("Failed to get settings.");
    let log_path = s.log_path
        .as_ref()
        .map(|s|s.as_str())
        .unwrap_or(DEFAULT_LOG_PATH);
    let _guard = setup_logger(log_path, LOG_NAME);
    info!("Startup ok");
    dbg!(s);
    dbg!(watcher::contains_unfinished_downloads("."));
}
