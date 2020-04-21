use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("A network error occured: {0}")]
    NetworkError(#[from] reqwest::Error),
    // hopefully rare errors
    #[error("FATAL: Failed to save state: {0}")]
    SerializeError(#[from] ron::ser::Error),
    #[error("FATAL: Failed to load state: {0}")]
    DeserializeError(#[from] ron::de::Error),
    #[error("Failed to parse News: {0}")]
    RssError(#[from] rss::Error),
    #[error("Failed to open browser: {0}")]
    OpenerError(#[from] opener::OpenError),
}
