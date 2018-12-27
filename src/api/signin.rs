use serde_derive::{Deserialize, Serialize};

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
    pub poll: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct SigninRequest {
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
#[serde(untagged)]
pub enum PollInteractiveSigninResponse {
    PollMoreResult(PollMoreResult),
    LoggedIn(LoggedInResponse),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PollMoreResult {
    pub retryAfter: u64
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SigninLinksResponse {
    pub interactiveLinkTimeout: u64,
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
