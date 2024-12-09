use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize, Error)]
#[error("code: {code}, msg: {msg}")]
pub struct BinanceContentError {
    pub code: i32,
    pub msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(reqwest::StatusCode),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid Price")]
    InvalidPrice,
    #[error("{response}")]
    BinanceError {
        #[from]
        response: BinanceContentError,
    },
    #[error("invalid listen key : {0}")]
    InvalidListenKey(String),
    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = std::result::Result<T, CustomError>;
