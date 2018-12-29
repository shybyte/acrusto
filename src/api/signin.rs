use serde_derive::{Deserialize, Serialize};
use crate::api::common_types::SuccessResponse;
use crate::api::common_types::NoLinks;

type AccessToken = String;
type UserId = String;
type Username = String;

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
pub struct SigninLinksData {
    pub interactiveLinkTimeout: u64,
}

type SigninLinksResponse = SuccessResponse<SigninLinksData, SigninLinks>;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LoggedInData {
    pub accessToken: AccessToken,
    pub user: User,
    pub authorizedUsing: AuthorizationType,
}

pub type LoggedInResponse = SuccessResponse<LoggedInData, NoLinks>;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub username: Username
}

