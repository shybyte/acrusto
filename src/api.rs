use reqwest;
use reqwest::{Error, StatusCode};
use reqwest::header::{ContentType, Headers, UserAgent};
use reqwest::mime::APPLICATION_JSON;

use serde_json;

header! { (XAcrolinxClientLocale, "X-Acrolinx-Client-Locale") => [String] }
header! { (XAcrolinxAuth, "X-Acrolinx-Auth") => [String] }
header! { (XAcrolinxBaseUrl, "X-Acrolinx-Base-Url") => [String] }

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ServerVersionInfo {
    version: String,
    buildNumber: String,
    buildDate: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct LoginRequest {
    clientName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum LoginRequestResponse {
    LoginLinks(LoginLinksResponse),
    LoggedIn(LoggedInResponse),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LoginLinksResponse {
    pub links: LoginLinks,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LoggedInResponse {
    pub authToken: String,
    pub userId: String,
    pub authorizedUsing: AuthorizationType,
    pub privileges: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub enum AuthorizationType {
    ACROLINX_SSO,
    ACROLINX_SIGN_IN,
    ACROLINX_TOKEN,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LoginLinks {
    pub interactive: String,
    poll: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub message: String,
}

pub struct AcroApi {
    server_url: String
}

impl AcroApi {
    pub fn new<S: Into<String>>(server_url: S) -> Self {
        AcroApi { server_url: server_url.into() }
    }

    pub fn server_version(&self) -> Result<ServerVersionInfo, Error> {
        let url = self.server_url.clone() + "/iq/services/v3/rest/core/serverVersion";
        let mut resp = reqwest::get(&url)?;
        // eprintln!("resp.text() = {:?}", resp.text().unwrap());

        resp.json()
    }

    pub fn login(&self, auth_token_option: Option<String>) -> Result<LoginRequestResponse, Error> {
        let client = reqwest::Client::new();
        let url = self.server_url.clone() + "/iq/services/v1/rest/login";

        let body = LoginRequest { clientName: "Acrusto".to_string() };

        let mut res = client.post(&url)
            .headers(self.get_headers(auth_token_option))
            .body(serde_json::to_string(&body).unwrap())
            .send()?;
        // eprintln!("Status = {:?}", res.status());
        // eprintln!("res.text() = {:?}", res.text().unwrap());

        res.json()
    }

    fn get_headers(&self, auth_token_option: Option<String>) -> Headers {
        let mut headers = Headers::new();
        headers.set(UserAgent::new("acrusto"));
        headers.set(ContentType(APPLICATION_JSON));
        headers.set(XAcrolinxBaseUrl(self.server_url.to_string()));
        headers.set(XAcrolinxClientLocale("en".to_string()));
        if let Some(auth_token) = auth_token_option {
            headers.set(XAcrolinxAuth(auth_token));
        };
        headers
    }

    pub fn wait_for_signin(&self, login_links: &LoginLinks) -> Result<LoggedInResponse, Error> {
        let mut res = reqwest::Client::new().get(&login_links.poll)
            .headers(self.get_headers(None))
            .send()?;

        while res.status() == StatusCode::Accepted  {
            res = reqwest::get(&login_links.poll)?;
        }

        res.json()
    }
}