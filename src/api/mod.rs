use reqwest;
use reqwest::Error;
use reqwest::header::{CONTENT_TYPE, USER_AGENT, HeaderName};
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
use crate::api::common_types::InternalApiResponse;
use crate::api::common_types::SuccessResponse;
use crate::api::common_types::ApiPollResponse;
use hyper::HeaderMap;
use std::str::FromStr;

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

impl AcroApi {
    pub fn new(props: AcroApiProps, authentication: Option<&str>) -> Self {
        AcroApi { props, authentication: authentication.map(|s| s.to_string()) }
    }

    pub fn server_version(&self) -> Result<ServerVersionInfo, ApiError> {
        let url = self.props.server_url.clone() + "/iq/services/v3/rest/core/serverVersion";
        let server_info = self.get(&url)?.json()?;
        Ok(server_info)
    }

    pub fn signin(&self, options: SigninOptions) -> Result<SigninRequestResponse, Error> {
        let url = self.props.server_url.clone() + "/api/v1/auth/sign-ins";
        let body = SigninRequest {};
        self.post(&url, &body, self.create_signin_headers(options))?.json()
    }

    pub fn get_checking_capabilities(&self) -> Result<CheckingCapabilities, ApiError> {
        let ir: InternalApiResponse<CheckingCapabilities, CheckingCapabilitiesLinks> =
            self.get_from_path("/api/v1/checking/capabilities")?;
        match ir {
            InternalApiResponse::SuccessResponse(s) => Ok(s.data),
            InternalApiResponse::ProgressResponse(_) => Err(ApiError {
                _type: "unexpected_progress".to_string(),
                title: "Unexpected Progress".to_string(),
                detail: "Unexpected Progress".to_string(),
                status: None,
            }),
            InternalApiResponse::ErrorResponse(error) => Err(error.error)
        }
    }

    pub fn check(&self, check_request: &CheckRequest)
                 -> Result<SuccessResponse<CheckResponse, CheckResponseLinks>, ApiError> {
        let url = self.props.server_url.clone() + "/api/v1/checking/checks";
        self.post(&url, &check_request, HeaderMap::new())?.json().map_err(ApiError::from)
    }

    pub fn get_checking_result(&self, check_response_links: &CheckResponseLinks)
                               -> Result<ApiPollResponse<CheckResult, CheckResultLinks>, ApiError> {
        self.get(&check_response_links.result)?.json().map_err(ApiError::from)
    }

    pub fn poll_for_signin(&self, signin_links: &SigninLinks, poll_more: Option<&PollMoreResult>) -> Result<PollInteractiveSigninResponse, ApiError> {
        if let Some(pm) = poll_more {
            thread::sleep(Duration::from_secs(pm.retryAfter));
        }
        self.get(&signin_links.poll)?.json().map_err(ApiError::from)
    }

    pub fn wait_for_signin(&self, signin_links: &SigninLinks) -> Result<LoggedInResponse, ApiError> {
        let mut res = self.poll_for_signin(signin_links, None)?;

        while let PollInteractiveSigninResponse::PollMoreResult(poll_more) = res {
            eprintln!("Polling ");
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
            Ok(response)
        } else {
            Err(response.json()?)
        }
    }

    fn get_from_path<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let mut res = self.get(&(self.props.server_url.clone() + path))?;
        res.json().map_err(ApiError::from)
    }

    fn post<U: reqwest::IntoUrl, B: ?Sized>(&self, url: U, body: &B, headers: HeaderMap) -> reqwest::Result<reqwest::Response>
        where B: serde::Serialize
    {
        let response = reqwest::Client::new()
            .post(url)
            .headers(self.create_common_headers())
            .headers(headers)
            .body(serde_json::to_string(&body).unwrap())
            .send();

        eprintln!("response = {:?}", response);

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

    fn create_signin_headers(&self, options: SigninOptions) -> HeaderMap {
        let mut headers = HeaderMap::new();

        match options {
            SigninOptions::Sso(sso_options) => {
                if let Some(user_id) = sso_options.user_id {
                    let header_name = sso_options.username_key.as_ref().map_or("username", String::as_ref);
                    headers.insert(HeaderName::from_str(header_name).unwrap(), user_id.parse().unwrap());
                }
                if let Some(password) = sso_options.password {
                    let header_name = sso_options.password_key.as_ref().map_or("username", String::as_ref);
                    headers.insert(HeaderName::from_str(header_name).unwrap(), password.parse().unwrap(),
                    );
                }
            }
            SigninOptions::Token(token) => { headers.insert(HEADER_ACROLINX_AUTH, token.parse().unwrap()); }
            SigninOptions::InteractiveSignin => {}
        }

        headers
    }
}