use log::info;

use crate::commands::common::connect;

pub fn show_capabilities(server_address: &str, token: Option<&str>) {
    let api = connect(server_address, token);
    info!("{:?}", api.server_info());
    let capabilities = api.get_checking_capabilities().unwrap();
    println!("{}", serde_json::to_string_pretty(&capabilities).unwrap());
}