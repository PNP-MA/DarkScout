use thiserror::Error;

#[derive(Error, Debug)]
pub enum DarkScoutError {
    #[error("Network error occurred while fetching from {0}: {1}")]
    NetworkError(String, #[source] reqwest::Error),

    #[error("Failed to parse response from {0}: {1}")]
    ParseError(String, String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Authentication error for {0}: {1}")]
    AuthError(String, String),
}

pub type Result<T> = std::result::Result<T, DarkScoutError>;
