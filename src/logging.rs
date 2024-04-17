use tracing_subscriber::FmtSubscriber;
use tracing::Level;

pub fn init() {
    let log_path = shellexpand::tilde("~/.local/state/walls/log").into_owned();
    let file_appender = tracing_appender::rolling::daily(log_path, "wallsd.log");
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(file_appender)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::info!("Logging initialized");
}