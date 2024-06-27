use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("FromUtf8Error {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("Json {0}")]
    Json(#[from] serde_json::Error),
}
