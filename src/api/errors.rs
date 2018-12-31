use serde_derive::{Deserialize};
use std::convert::From;
use reqwest;
use std::error::Error;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
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
            status: request_error.status().map(|s| s.as_u16())
        }
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(serde_error: serde_json::error::Error) -> ApiError {
        ApiError {
            _type: "SerdeError".to_string(),
            title: serde_error.description().to_string(),
            detail: serde_error.to_string(),
            status: None
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(io_error: std::io::Error) -> ApiError {
        ApiError {
            _type: "IoError".to_string(),
            title: io_error.description().to_string(),
            detail: io_error.to_string(),
            status: None
        }
    }
}
