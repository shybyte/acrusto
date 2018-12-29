use clap::crate_version;
use crate::api::AcroApi;
use crate::api::AcroApiProps;
use crate::api::ClientInformation;
use crate::api::signin::SigninRequestResponse;
use log::info;

pub struct CommandConfig {
    pub acrolinx_address: String,
    pub access_token: Option<String>,
    pub silent: bool,
}

pub fn connect(config: &CommandConfig) -> AcroApi {
    AcroApi::new(AcroApiProps {
        server_url: config.acrolinx_address.clone(),
        locale: "en".to_string(),
        client: ClientInformation {
            name: "Acrusto".to_string(),
            signature: "dummyClientSignature".to_string(),
            version: crate_version!().to_string(),
        },
    }, config.access_token.clone())
}

pub fn connect_and_signin(config: &CommandConfig) -> AcroApi {
    let mut api = connect(&config);

    info!("Yeah, there is a server: {:?}", api.server_info());

    let signin_response = api.signin().unwrap();
    info!("signin_response = {:?}", signin_response);

    let signin_details = match signin_response {
        SigninRequestResponse::SigninLinks(signin_links_response) => {
            info!("Please signin at {:?}", signin_links_response.links.interactive);
            let logged_in = api.wait_for_signin(&signin_links_response.links).unwrap();
            info!("authToken = {:?}", logged_in.data.accessToken);
            api.set_access_token(&logged_in.data.accessToken);
            logged_in
        }
        SigninRequestResponse::LoggedIn(logged_in) => {
            info!("You are already logged in as {:?} ", logged_in.data.user.id);
            logged_in
        }
    };

    if !config.silent {
        println!("You're signed in as {:?}", signin_details.data.user.id);
    }

    api
}
