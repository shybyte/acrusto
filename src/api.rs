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
struct SigninRequest {
    clientName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SigninRequestResponse {
    SigninLinks(SigninLinksResponse),
    LoggedIn(LoggedInResponse),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SigninLinksResponse {
    pub links: SigninLinks,
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
pub struct SigninLinks {
    pub interactive: String,
    poll: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub message: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct SsoOptions {
    pub username_key: Option<String>,
    pub password_key: Option<String>,
    pub user_id: Option<String>,
    pub password: Option<String>,
}


pub enum SigninOptions {
    Sso(SsoOptions),
    Token(String),
    InteractiveSignin,
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

    pub fn signin(&self, options: SigninOptions) -> Result<SigninRequestResponse, Error> {
        let client = reqwest::Client::new();
        let url = self.server_url.clone() + "/api/v1/auth/sign-ins";

        let body = SigninRequest { clientName: "Acrusto".to_string() };

        let mut res = client.post(&url)
            .headers(self.get_headers(options))
            .body(serde_json::to_string(&body).unwrap())
            .send()?;
        // eprintln!("Status = {:?}", res.status());
        // eprintln!("res.text() = {:?}", res.text().unwrap());

        res.json()
    }

    fn get_headers(&self, options: SigninOptions) -> Headers {
        let mut headers = Headers::new();

        headers.set(UserAgent::new("acrusto"));
        headers.set(ContentType(APPLICATION_JSON));
        headers.set(XAcrolinxBaseUrl(self.server_url.to_string()));
        headers.set(XAcrolinxClientLocale("en".to_string()));

        match options {
            SigninOptions::Sso(sso_options) => {
                if let Some(user_id) = sso_options.user_id {
                    headers.set_raw(
                        sso_options.username_key.unwrap_or_else(|| "username".to_string()),
                        user_id
                    );
                }
                if let Some(password) = sso_options.password {
                    headers.set_raw(
                        sso_options.password_key.unwrap_or_else(|| "password".to_string()),
                        password
                    );
                }
            }
            SigninOptions::Token(token) => headers.set(XAcrolinxAuth(token)),
            SigninOptions::InteractiveSignin => {}
        }


        headers
    }

    pub fn wait_for_signin(&self, signin_links: &SigninLinks) -> Result<LoggedInResponse, Error> {
        let mut res = reqwest::Client::new().get(&signin_links.poll)
            .headers(self.get_headers(SigninOptions::InteractiveSignin))
            .send()?;

        while res.status() == StatusCode::Accepted {
            res = reqwest::get(&signin_links.poll)?;
        }

        res.json()
    }
}