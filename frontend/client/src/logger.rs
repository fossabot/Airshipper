use crate::config;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init(max_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub fn set_voxygen_log_level(level: Level) {
    std::env::set_var(config::VOXYGEN_LOG_ENV, level.to_string())
}
