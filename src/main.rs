mod config;
mod api;


use std::env;
use std::time::Duration;
use std::thread;
use clap::{Arg, App, SubCommand};
use crate::api::{AcroApi, AcroApiProps, ClientInformation};
use crate::api::signin::{SigninOptions, SsoOptions};
use crate::api::checking::{CheckRequest, DocumentInfo};
use crate::api::signin::SigninRequestResponse::*;
use crate::config::Config;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use clap::{crate_version};
use crate::api::common_types::ApiPollResponse;
use crate::api::checking::CheckOptions;
use log::{info, Level};
use simple_logger;


fn connect<S: Into<String>>(server_url: S, token: Option<&str>) -> AcroApi {
    AcroApi::new(AcroApiProps {
        server_url: server_url.into(),
        locale: "en".to_string(),
        client: ClientInformation {
            name: "Acrusto".to_string(),
            signature: "dummyClientSignature".to_string(),
            version: crate_version!().to_string(),
        },
    }, token)
}


fn server_info(server_address: &str, token_option: Option<&str>) {
    let api = connect(server_address, token_option);
    println!("{}", serde_json::to_string_pretty(&api.server_info().unwrap()).unwrap());
}

fn check(server_address: &str, filename: &str, token: Option<&str>) {
    let api = connect(server_address, token);
    info!("{:?}", api.server_info());
    let capabilities = api.get_checking_capabilities().unwrap();
    info!("{:?}", capabilities);

    let mut f = File::open(filename).expect("File not found");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("Problem reading document");
    info!("file_content = {:?}", file_content);

    let check_request = CheckRequest {
        content: file_content,
        checkOptions: CheckOptions { guidanceProfileId: capabilities.guidanceProfiles.first().map(|a| a.id.clone()) },
        document: Some(DocumentInfo {
            reference: fs::canonicalize(filename).ok()
                .map(|path| path.to_string_lossy().into_owned())
        }),
    };
    let check = api.check(&check_request).unwrap();

    let mut check_poll_response;
    let check_result;
    loop {
        check_poll_response = api.get_checking_result(&check.links).unwrap();
        info!("check_poll_response = {:?}", check_poll_response);
        match check_poll_response {
            ApiPollResponse::SuccessResponse(s) => {
                check_result = s.data;
                break;
            }
            ApiPollResponse::ProgressResponse(p) => {
                info!("progress = {:?}", p.progress.percent);
                thread::sleep(Duration::from_secs(p.progress.retryAfter));
            }
        }
    }

    info!("check_result = {:?}", check_result);
}

fn signin_command(server_address: &str, auth_token_option: Option<String>) {
    let api = connect(server_address, None);

    info!("Yeah, there is a server: {:?}", api.server_info());

    let signin_options = match auth_token_option {
        Some(token) => SigninOptions::Token(token),
        None => SigninOptions::InteractiveSignin
    };

    let signin_response = api.signin(signin_options).unwrap();
    info!("signin_response = {:?}", signin_response);

    match signin_response {
        SigninLinks(signin_links_response) => {
            info!("Please signin at {:?}", signin_links_response.links.interactive);
            let logged_in = api.wait_for_signin(&signin_links_response.links).unwrap();
            info!("authToken = {:?}", logged_in.authToken);
            info!("You are logged in as {:?}", logged_in.userId);
        }
        LoggedIn(logged_in) => {
            info!("You are already logged in as {:?}", logged_in.userId);
        }
    }
}

fn sso_command<S: Into<String>>(server_address: &str, user_id: S, password: S) {
    let api = connect(server_address, None);
    info!("Yeah, there is a server: {:?}", api.server_info());
    let signin_response = api.signin(SigninOptions::Sso(
        SsoOptions {
            user_id: Some(user_id.into()),
            password: Some(password.into()),
            ..SsoOptions::default()
        })).unwrap();
    info!("signin_response = {:?}", signin_response);
}

fn create_arg<'a, 'b>(name: &'a str, env_var_name: &'a str, default_option: &'a Option<String>) -> Arg<'a, 'b> {
    let arg = Arg::with_name(name)
        .long(name)
        .env(env_var_name);

    if let Some(ref default_value) = *default_option {
        arg.default_value(default_value)
    } else {
        arg
    }
}

static SERVER_ADDRESS_ARG: &str = "serverAddress";
static SILENT_FLAG: &str = "silent";
static USER_ID_ARG: &str = "USER_ID";
static PASSWORD_ARG: &str = "PASSWORD";
static AUTH_TOKEN_ARG: &str = "authToken";

static DOCUMENT_ARG: &str = "DOCUMENT";

static SUB_COMMAND_SIGN_IN: &str = "signin";
static SUB_COMMAND_SSO: &str = "sso";
static SUB_COMMAND_INFO: &str = "info";
static SUB_COMMAND_CHECK: &str = "check";


fn main() {
    let config = Config::read();

    let auth_token_arg = create_arg(AUTH_TOKEN_ARG, "ACROLINX_AUTH_TOKEN", &config.access_token)
        .short("t")
        .help("Use an authToken")
        .takes_value(true);

    let server_address_arg = create_arg(SERVER_ADDRESS_ARG, "ACROLINX_SERVER_ADDRESS", &config.acrolinx_address)
        .short("a")
        .required(true)
        .takes_value(true);

    let silent_flag = create_arg(SILENT_FLAG, "ACROLINX_SILENT", &None)
        .short("s")
        .takes_value(false);

    let mut command_line_parser = App::new("acrusto")
        .version(crate_version!())
        .author("Marco Stahl <shybyte@gmail.com>")
        .about("Unofficial commandline tool for the Acrolinx Platform API")
        .arg(server_address_arg)
        .arg(auth_token_arg)
        .arg(silent_flag)
        .subcommand(SubCommand::with_name(SUB_COMMAND_SIGN_IN).about("Signin to Acrolinx"))
        .subcommand(SubCommand::with_name(SUB_COMMAND_INFO).about("Show server information"))
        .subcommand(SubCommand::with_name(SUB_COMMAND_SSO)
            .about("Signin to Acrolinx by SSO")
            .arg(Arg::with_name(USER_ID_ARG).required(true).index(2))
            .arg(Arg::with_name(PASSWORD_ARG).required(true).index(3))
        ).subcommand(SubCommand::with_name(SUB_COMMAND_CHECK)
        .about("Check a document")
        .arg(Arg::with_name(DOCUMENT_ARG).required(true).index(1))
    );

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        command_line_parser.print_help().ok();
    }

    let matches = command_line_parser.get_matches();
    let auth_token_option = matches.value_of(AUTH_TOKEN_ARG);
    let server_address = matches.value_of(SERVER_ADDRESS_ARG).unwrap();

    if !matches.is_present(SILENT_FLAG) {
        simple_logger::init_with_level(Level::Info).ok();
    }

    if matches.subcommand_matches(SUB_COMMAND_SIGN_IN).is_some() {
        info!("signin {:?} {:?}", server_address, auth_token_option);
        signin_command(server_address, auth_token_option.map(|s| s.to_string()));
    } else if matches.subcommand_matches(SUB_COMMAND_INFO).is_some() {
        info!("info {:?} {:?}", server_address, auth_token_option);
        server_info(server_address, auth_token_option);
    } else if let Some(command_matches) = matches.subcommand_matches(SUB_COMMAND_SSO) {
        let user_id = command_matches.value_of(USER_ID_ARG).unwrap();
        let password = command_matches.value_of(PASSWORD_ARG).unwrap();
        info!("sso {:?} {:?} {:?}", server_address, user_id, password);
        sso_command(server_address, user_id, password);
    } else if let Some(command_matches) = matches.subcommand_matches(SUB_COMMAND_CHECK) {
        let document_file_name = command_matches.value_of(DOCUMENT_ARG).unwrap();
        info!("check {:?} {:?} {:?}", server_address, document_file_name, auth_token_option);
        check(server_address, document_file_name, auth_token_option);
    }
}
