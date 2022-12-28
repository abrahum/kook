use thiserror::Error;

pub type KookResult<T> = Result<T, KookError>;

#[derive(Debug, Error)]
pub enum KookError {
    #[error("hyper error:{0}")]
    HyperError(#[from] hyper::Error),
    #[error("serde_json error:{0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("KHL Api error:{0}")]
    HttpApiCallError(String),
    #[error("KHL Api get empty response")]
    HttpApiCallEmptyResponse,
    #[error("reqwest error:{0}")]
    ReqwestError(#[from] reqwest::Error),
}
