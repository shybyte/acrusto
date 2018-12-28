use crate::commands::common::connect;

pub fn server_info(server_address: &str, token_option: Option<&str>) {
    let api = connect(server_address, token_option);
    println!("{}", serde_json::to_string_pretty(&api.server_info().unwrap()).unwrap());
}
