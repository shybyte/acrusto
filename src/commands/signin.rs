use crate::commands::common::CommonCommandConfig;
use crate::commands::common::connect_and_signin;


pub fn signin_command(config: &CommonCommandConfig) {
    connect_and_signin(&config);
}
