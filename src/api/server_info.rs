use serde_derive::{Deserialize, Serialize};
use crate::api::common_types::Locale;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct ServerVersionInfo {
    version: String,
    name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct ServerInfo {
    server: ServerVersionInfo,
    locales: Vec<Locale>,
}
