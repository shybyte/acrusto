use crate::commands::common::connect;
use crate::api::signin::SigninRequestResponse::SigninLinks;
use crate::api::signin::SigninRequestResponse::LoggedIn;
use log::{info};
use crate::commands::common::CommandConfig;


pub fn signin_command(config: CommandConfig) {
    let api = connect(&config);

    info!("Yeah, there is a server: {:?}", api.server_info());

    let signin_response = api.signin().unwrap();
    info!("signin_response = {:?}", signin_response);

    match signin_response {
        SigninLinks(signin_links_response) => {
            info!("Please signin at {:?}", signin_links_response.links.interactive);
            let logged_in = api.wait_for_signin(&signin_links_response.links).unwrap();
            info!("authToken = {:?}", logged_in.data.accessToken);
            info!("You are logged in as {:?}", logged_in.data.user.id);
        }
        LoggedIn(logged_in) => {
            info!("You are already logged in as {:?} ", logged_in.data.user.id);
        }
    }
}
