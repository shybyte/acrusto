use serde_derive::{Deserialize};
use crate::api::errors::ApiError;

pub type Locale = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SuccessResponse<Data, Links> {
    pub data: Data,
    pub links: Links,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct NoLinks {
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ApiError
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ProgressResponse {
    pub progress: Progress,
    pub links: ProgressResponseLinks,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ProgressResponseLinks {
    pub poll: Option<String>
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Progress {
    pub percent: Option<f64>,
    pub message: Option<String>,

    /// seconds
    pub retryAfter: u64
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum InternalApiResponse<Data, Links> {
    SuccessResponse(SuccessResponse<Data, Links>),
    ProgressResponse(ProgressResponse),
    ErrorResponse(ErrorResponse),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiPollResponse<Data, Links> {
    SuccessResponse(SuccessResponse<Data, Links>),
    ProgressResponse(ProgressResponse),
}