use std::convert::From;
use std::error::Error;

use lazy_static::lazy_static;
use reqwest;
use serde_derive::Deserialize;

pub static CHECK_CANCELLED_ERROR_TYPE: &str = "checkCancelled";

lazy_static! {
    pub static ref CHECK_CANCELLED_ERROR: ApiError = ApiError {
        _type: CHECK_CANCELLED_ERROR_TYPE.to_string(),
        title: "Check cancelled".to_string(),
        detail: "".to_string(),
        status: None,
    };
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub _type: String,
    pub title: String,
    pub detail: String,
    pub status: Option<u16>,
}

impl From<reqwest::Error> for ApiError {
    fn from(request_error: reqwest::Error) -> ApiError {
        ApiError {
            _type: "RequestError".to_string(),
            title: request_error.description().to_string(),
            detail: request_error.to_string(),
            status: request_error.status().map(|s| s.as_u16()),
        }
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(serde_error: serde_json::error::Error) -> ApiError {
        ApiError {
            _type: "SerdeError".to_string(),
            title: serde_error.description().to_string(),
            detail: serde_error.to_string(),
            status: None,
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(io_error: std::io::Error) -> ApiError {
        ApiError {
            _type: "IoError".to_string(),
            title: io_error.description().to_string(),
            detail: io_error.to_string(),
            status: None,
        }
    }
}
