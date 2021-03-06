use reqwest;
use reqwest::Error;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use mime::APPLICATION_JSON;
use serde;
use std::time::Duration;
use std::thread;

use serde_json;
use serde::de::DeserializeOwned;

pub mod checking;
pub mod errors;
pub mod server_info;
pub mod signin;
pub mod common_types;

use self::checking::*;
use self::server_info::*;
use self::signin::*;
use self::errors::ApiError;
use crate::api::common_types::SuccessResponse;
use crate::api::common_types::ApiPollResponse;
use hyper::HeaderMap;
use crate::api::common_types::ErrorResponse;
use log::info;
use crate::api::common_types::NoLinks;

const HEADER_ACROLINX_CLIENT_LOCALE: &str = "X-Acrolinx-Client-Locale";
const HEADER_ACROLINX_AUTH: &str = "X-Acrolinx-Auth";
const HEADER_ACROLINX_BASE_URL: &str = "X-Acrolinx-Base-Url";
const HEADER_ACROLINX_CLIENT: &str = "X-Acrolinx-Client";

pub struct AcroApi {
    props: AcroApiProps,
    authentication: Option<String>,
}

pub struct AcroApiProps {
    pub server_url: String,
    pub locale: String,
    pub client: ClientInformation,
}

pub struct ClientInformation {
    pub name: String,
    pub version: String,
    pub signature: String,
}

const API_BASE_PATH: &str = "/api/v1";

impl AcroApi {
    pub fn new<S: Into<String>>(props: AcroApiProps, authentication: Option<S>) -> Self {
        AcroApi { props, authentication: authentication.map(Into::into) }
    }

    pub fn set_access_token(&mut self, access_token: &str) {
        self.authentication = Some(access_token.to_string());
    }

    pub fn server_info(&self) -> Result<ServerInfo, ApiError> {
        self.get_data("")
    }

    pub fn signin(&self) -> Result<SigninRequestResponse, Error> {
        let url = self.props.server_url.clone() + "/api/v1/auth/sign-ins";
        let body = SigninRequest {};
        self.post(&url, &body)?.json()
    }

    pub fn get_checking_capabilities(&self) -> Result<CheckingCapabilities, ApiError> {
        self.get_data("/checking/capabilities")
    }

    pub fn check(&self, check_request: &CheckRequest)
                 -> Result<SuccessResponse<CheckResponse, CheckResponseLinks>, ApiError> {
        let url = self.props.server_url.clone() + "/api/v1/checking/checks";
        self.post(&url, &check_request)?.json().map_err(ApiError::from)
    }

    pub fn cancel_check(&self, check_response_links: &CheckResponseLinks)
                        -> Result<SuccessResponse<CancelCheckResponseData, NoLinks>, ApiError> {
        self.delete(&check_response_links.cancel)?.json().map_err(ApiError::from)
    }

    pub fn get_checking_result(&self, check_response_links: &CheckResponseLinks)
                               -> Result<ApiPollResponse<CheckResult, CheckResultLinks>, ApiError> {
        self.get(&check_response_links.result)?.json().map_err(ApiError::from)
    }

    pub fn get_link_to_aggregated_report(&self, batch_id: &str)
                                         -> Result<AggregatedReportLinkResponse, ApiError> {
        let url = self.props.server_url.clone() + "/api/v1/checking/aggregation/" + batch_id;
        self.get(&url)?.json().map_err(ApiError::from)
    }

    pub fn poll_for_signin(&self, signin_links: &SigninLinks, poll_more: Option<&PollMoreResult>) -> Result<PollInteractiveSigninResponse, ApiError> {
        if let Some(pm) = poll_more {
            thread::sleep(Duration::from_secs(pm.progress.retryAfter));
        }
        self.get(&signin_links.poll)?.json().map_err(ApiError::from)
    }

    pub fn wait_for_signin(&self, signin_links: &SigninLinks) -> Result<LoggedInResponse, ApiError> {
        let mut res = self.poll_for_signin(signin_links, None)?;

        while let PollInteractiveSigninResponse::PollMoreResult(poll_more) = res {
            info!("Polling ");
            res = self.poll_for_signin(signin_links, Some(&poll_more))?;
        }

        if let PollInteractiveSigninResponse::LoggedIn(signed_in) = res {
            Ok(signed_in)
        } else {
            panic!("This should never happen.");
        }
    }

    fn get_raw<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::Result<reqwest::Response> {
        reqwest::Client::new()
            .get(url)
            .headers(self.create_common_headers())
            .send()
    }

    fn get<U: reqwest::IntoUrl>(&self, url: U) -> Result<reqwest::Response, ApiError> {
        let mut response = self.get_raw(url)?;
        if response.status().is_success() {
            info!("response = {:?}", response);
            Ok(response)
        } else {
            let error_response: ErrorResponse = response.json()?;
            Err(error_response.error)
        }
    }

    fn get_data<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = self.props.server_url.clone() + API_BASE_PATH + path;
        let mut response_raw = self.get_raw(&url)?;
        if response_raw.status().is_success() {
            info!("response_raw = {:?}", response_raw);
            let response: SuccessResponse<T, NoLinks> = response_raw.json()?;
            Ok(response.data)
        } else {
            let error_response: ErrorResponse = response_raw.json()?;
            Err(error_response.error)
        }
    }

    fn post<U: reqwest::IntoUrl, B: ?Sized>(&self, url: U, body: &B) -> reqwest::Result<reqwest::Response>
        where B: serde::Serialize
    {
        let response = reqwest::Client::new()
            .post(url)
            .headers(self.create_common_headers())
            .body(serde_json::to_string(&body).unwrap())
            .send();

        info!("response = {:?}", response);

        response
    }

    fn delete<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::Result<reqwest::Response> {
        let response = reqwest::Client::new()
            .delete(url)
            .headers(self.create_common_headers())
            .send();

        info!("response = {:?}", response);

        response
    }

    fn create_common_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, self.props.client.name.parse().unwrap());
        headers.insert(CONTENT_TYPE, APPLICATION_JSON.to_string().parse().unwrap());
        headers.insert(HEADER_ACROLINX_BASE_URL, self.props.server_url.parse().unwrap());
        headers.insert(HEADER_ACROLINX_CLIENT_LOCALE, self.props.locale.parse().unwrap());
        headers.insert(HEADER_ACROLINX_CLIENT, format!("{}; {}", self.props.client.signature, self.props.client.version).parse().unwrap());

        if let Some(ref token) = self.authentication {
            headers.insert(HEADER_ACROLINX_AUTH, token.parse().unwrap());
        }

        headers
    }
}