use crate::commands::common::connect;
use crate::commands::common::CommandConfig;

pub fn server_info(config: CommandConfig) {
    let api = connect(&config);
    println!("{}", serde_json::to_string_pretty(&api.server_info().unwrap()).unwrap());
}
