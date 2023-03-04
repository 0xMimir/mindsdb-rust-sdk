use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serde json error: {:?}", self)]
    SerdeJsonError(#[from] serde_json::Error),
    
    #[error("Reqwest error: {:?}", self)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error response: {}", self)]
    InternalError(String),

    #[error("Not found")]
    NotFound,
}
