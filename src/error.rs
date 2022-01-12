use thiserror::Error;

pub type KHLResult<T> = Result<T, KHLError>;

#[derive(Debug, Error)]
pub enum KHLError {
    #[error("reqwest error:{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("serde_json error:{0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("KHL Api error:{0}")]
    HttpApiCallError(String),
}
