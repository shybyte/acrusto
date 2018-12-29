use crate::commands::common::connect;
use crate::commands::common::CommonCommandConfig;

pub fn server_info(config: CommonCommandConfig) {
    let api = connect(&config);
    println!("{}", serde_json::to_string_pretty(&api.server_info().unwrap()).unwrap());
}
