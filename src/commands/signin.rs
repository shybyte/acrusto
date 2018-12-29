use crate::commands::common::CommandConfig;
use crate::commands::common::connect_and_signin;


pub fn signin_command(config: CommandConfig) {
    connect_and_signin(&config);
}
