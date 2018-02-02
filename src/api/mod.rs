use reqwest;
use reqwest::{Error, StatusCode};
use reqwest::header::{ContentType, Headers, UserAgent, RetryAfter};
use reqwest::mime::APPLICATION_JSON;
use serde;
use std::time::Duration;
use std::thread;

use serde_json;

pub mod checking;
pub mod errors;
pub mod server_info;
pub mod signin;

use self::checking::*;
use self::server_info::*;
use self::signin::*;
use self::errors::ApiError;

header! { (XAcrolinxClientLocale, "X-Acrolinx-Client-Locale") => [String] }
header! { (XAcrolinxAuth, "X-Acrolinx-Auth") => [String] }
header! { (XAcrolinxBaseUrl, "X-Acrolinx-Base-Url") => [String] }
header! { (XAcrolinxClient, "X-Acrolinx-Client") => [String] }


pub struct AcroApi {
    props: AcroApiProps
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
    pub fn new(props: AcroApiProps) -> Self {
        AcroApi { props }
    }

    pub fn server_version(&self) -> Result<ServerVersionInfo, ApiError> {
        let url = self.props.server_url.clone() + "/iq/services/v3/rest/core/serverVersion";
        let server_info = self.get(&url, None)?.json()?;
        Ok(server_info)
    }

    pub fn signin(&self, options: SigninOptions) -> Result<SigninRequestResponse, Error> {
        let url = self.props.server_url.clone() + "/api/v1/auth/sign-ins";
        let body = SigninRequest {};
        self.post(&url, &body, self.create_signin_headers(options))?.json()
    }

    pub fn get_checking_capabilities(&self, token: &str) -> Result<CheckingCapabilities, ApiError> {
        let url = self.props.server_url.clone() + "/api/v1/checking/capabilities";
        self.get(&url, Some(token))?.json().map_err(ApiError::from)
    }

    pub fn poll_for_signin(&self, signin_links: &SigninLinks, poll_more: Option<&PollMoreResult>) -> Result<PollInteractiveSigninResponse, ApiError> {
        if let Some(pm) = poll_more {
            thread::sleep(Duration::from_secs(pm.retryAfter));
        }
        self.get(&signin_links.poll, None)?.json().map_err(ApiError::from)
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

    fn get_raw<U: reqwest::IntoUrl>(&self, url: U, token: Option<&str>) -> reqwest::Result<reqwest::Response> {
        reqwest::Client::new()
            .get(url)
            .headers(self.create_common_headers(token))
            .send()
    }

    fn get<U: reqwest::IntoUrl>(&self, url: U, token: Option<&str>) -> Result<reqwest::Response, ApiError> {
        let mut response = self.get_raw(url, token)?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(response.json()?)
        }
    }

    fn post<U: reqwest::IntoUrl, B: ? Sized>(&self, url: U, body: &B, headers: Headers) -> reqwest::Result<reqwest::Response>
        where B: serde::Serialize
    {
        let response = reqwest::Client::new()
            .post(url)
            .headers(self.create_common_headers(None))
            .headers(headers)
            .body(serde_json::to_string(&body).unwrap())
            .send();

        eprintln!("response = {:?}", response);

        response
    }

    fn create_common_headers(&self, token: Option<&str>) -> Headers {
        let mut headers = Headers::new();
        headers.set(UserAgent::new(self.props.client.name.clone()));
        headers.set(ContentType(APPLICATION_JSON));
        headers.set(XAcrolinxBaseUrl(self.props.server_url.to_string()));
        headers.set(XAcrolinxClientLocale(self.props.locale.to_string()));
        headers.set(XAcrolinxClient(format!("{}; {}", self.props.client.signature, self.props.client.version)));
        if let Some(token) = token {
            headers.set(XAcrolinxAuth(token.to_string()));
        }
        headers
    }

    fn create_signin_headers(&self, options: SigninOptions) -> Headers {
        let mut headers = Headers::new();

        match options {
            SigninOptions::Sso(sso_options) => {
                if let Some(user_id) = sso_options.user_id {
                    headers.set_raw(
                        sso_options.username_key.unwrap_or_else(|| "username".to_string()),
                        user_id,
                    );
                }
                if let Some(password) = sso_options.password {
                    headers.set_raw(
                        sso_options.password_key.unwrap_or_else(|| "password".to_string()),
                        password,
                    );
                }
            }
            SigninOptions::Token(token) => headers.set(XAcrolinxAuth(token)),
            SigninOptions::InteractiveSignin => {}
        }

        headers
    }
}