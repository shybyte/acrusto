use reqwest;
use reqwest::{Error, StatusCode};
use reqwest::header::ContentType;
use reqwest::mime::APPLICATION_JSON;

use serde_json;

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

    pub fn login(&self) -> Result<LoginRequestResponse, Error> {
        let client = reqwest::Client::new();
        let url = self.server_url.clone() + "/iq/services/v1/rest/login";

        let body = LoginRequest { clientName: "Acrusto".to_string() };

        let mut res = client.post(&url)
            .header(ContentType(APPLICATION_JSON))
            .body(serde_json::to_string(&body).unwrap())
            .send()?;
        // eprintln!("Status = {:?}", res.status());
        // eprintln!("res.text() = {:?}", res.text().unwrap());

        res.json()
    }

    pub fn wait_for_signin(&self, login_links: &LoginLinks) -> Result<LoggedInResponse, Error> {
        let mut res = reqwest::get(&login_links.poll)?;
        while res.status() == StatusCode::Accepted  {
            res = reqwest::get(&login_links.poll)?;
        }
        return res.json();
    }
}