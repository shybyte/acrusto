use clap::crate_version;
use crate::api::AcroApi;
use crate::api::AcroApiProps;
use crate::api::ClientInformation;
use crate::api::signin::SigninRequestResponse;
use log::info;
use crate::api::signin::LoggedInData;
use crate::utils::open_url;

pub struct CommandConfig {
    pub acrolinx_address: String,
    pub access_token: Option<String>,
    pub silent: bool,
    pub open_url: bool,
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

pub struct ConnectAndSigninResult {
    pub api: AcroApi,
    pub signin_details: LoggedInData,
}

pub fn connect_and_signin(config: &CommandConfig) -> ConnectAndSigninResult {
    let mut api = connect(&config);

    info!("Yeah, there is a server: {:?}", api.server_info());

    let signin_response = api.signin().unwrap();
    info!("signin_response = {:?}", signin_response);

    let signin_details = match signin_response {
        SigninRequestResponse::SigninLinks(signin_links_response) => {
            if !config.silent {
                println!("Please signin at");
            }
            println!("{}", signin_links_response.links.interactive);

            if config.open_url {
                open_url(&signin_links_response.links.interactive).unwrap();
            }

            let signin_details = api.wait_for_signin(&signin_links_response.links).unwrap();

            if config.silent {
                // TODO: As JSON?
                println!("{}", signin_details.data.accessToken);
            } else {
                println!("You can use the following token to sign in: ");
                println!("{}\n", signin_details.data.accessToken);
            }

            api.set_access_token(&signin_details.data.accessToken);
            signin_details
        }
        SigninRequestResponse::LoggedIn(signin_details) => {
            info!("You are already logged in as {:?} ", signin_details.data.user.id);
            signin_details
        }
    };

    if !config.silent {
        println!("You're signed in as \"{}\"", signin_details.data.user.username);
    }

    ConnectAndSigninResult { api, signin_details: signin_details.data }
}
