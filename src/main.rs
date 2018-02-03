#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate hyper;


mod config;
mod api;


use std::env;
use std::time::Duration;
use std::thread;
use clap::{Arg, App, SubCommand};
use api::{AcroApi, AcroApiProps, ClientInformation};
use api::signin::{SigninOptions, SsoOptions};
use api::checking::CheckRequest;
use api::signin::SigninRequestResponse::*;
use config::Config;
use std::fs::File;
use std::io::prelude::*;


fn connect<S: Into<String>>(server_url: S) -> AcroApi {
    AcroApi::new(AcroApiProps {
        server_url: server_url.into(),
        locale: "en".to_string(),
        client: ClientInformation {
            name: "Acrusto".to_string(),
            signature: "dummySignature".to_string(),
            version: crate_version!().to_string(),
        },
    })
}


fn server_info(server_address: &str, token_option: Option<&str>) {
    let api = connect(server_address);
    println!("{:?}", api.server_version());
    if token_option.is_some() {
        println!("{:?}", api.get_checking_capabilities(token_option));
    }
}

fn check(server_address: &str, filename: &str, token: Option<&str>) {
    let api = connect(server_address);
    println!("{:?}", api.server_version());
    println!("{:?}", api.get_checking_capabilities(token));

    let mut f = File::open(filename).expect("File not found");

    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("Problem reading document");

    eprintln!("file_content = {:?}", file_content);

    let check = api.check(token, &CheckRequest { content: file_content }).unwrap();

    let mut checking_status;
    loop {
        checking_status = api.get_checking_status(token, &check).unwrap();
        if checking_status.state == "done" {
            break;
        }
        eprintln!("checking_status = {:?}", checking_status);
        thread::sleep(Duration::from_secs(1));
    }

    let check_result = api.get_checking_result(token, &check).unwrap();

    eprintln!("check_result = {:?}", check_result);
}

fn signin_command(server_address: &str, auth_token_option: Option<String>) {
    let api = connect(server_address);

    println!("Yeah, there is a server: {:?}", api.server_version());

    let signin_options = match auth_token_option {
        Some(token) => SigninOptions::Token(token),
        None => SigninOptions::InteractiveSignin
    };

    let signin_response = api.signin(signin_options).unwrap();
    println!("signin_response = {:?}", signin_response);

    match signin_response {
        SigninLinks(signin_links_response) => {
            println!("Please signin at {:?}", signin_links_response.links.interactive);
            let logged_in = api.wait_for_signin(&signin_links_response.links).unwrap();
            println!("authToken = {:?}", logged_in.authToken);
            println!("You are logged in as {:?}", logged_in.userId);
        }
        LoggedIn(logged_in) => {
            println!("You are already logged in as {:?}", logged_in.userId);
        }
    }
}

fn sso_command<S: Into<String>>(server_address: &str, user_id: S, password: S) {
    let api = connect(server_address);
    println!("Yeah, there is a server: {:?}", api.server_version());
    let signin_response = api.signin(SigninOptions::Sso(
        SsoOptions {
            user_id: Some(user_id.into()),
            password: Some(password.into()),
            ..SsoOptions::default()
        })).unwrap();
    println!("signin_response = {:?}", signin_response);
}

static SERVER_ADDRESS_ARG: &str = "serverAddress";
static USER_ID_ARG: &str = "USER_ID";
static PASSWORD_ARG: &str = "PASSWORD";
static AUTH_TOKEN_ARG: &str = "authToken";

static DOCUMENT_ARG: &str = "DOCUMENT";

static SUB_COMMAND_SIGN_IN: &str = "signin";
static SUB_COMMAND_SSO: &str = "sso";
static SUB_COMMAND_INFO: &str = "info";
static SUB_COMMAND_CHECK: &str = "check";


fn main() {
    let config = Config::read().unwrap();
    let auth_token_option = {
        let arg = Arg::with_name(AUTH_TOKEN_ARG)
            .short("a")
            .long(AUTH_TOKEN_ARG)
            .help("Use an authToken")
            .takes_value(true);
        if let Some(ref auth_token) = config.authToken {
            arg.default_value(auth_token)
        } else {
            arg
        }
    };

    let mut command_line_parser = App::new("acrusto")
        .version(crate_version!())
        .author("Marco Stahl <shybyte@gmail.com>")
        .about("Unofficial commandline tool for the Acrolinx Platform API")
        .arg({
            let arg = Arg::with_name(SERVER_ADDRESS_ARG).required(true).long(SERVER_ADDRESS_ARG).takes_value(true);
            if let Some(ref server_address) = config.serverAddress {
                arg.default_value(server_address)
            } else {
                arg
            }
        })
        .arg(auth_token_option.clone())
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

    if matches.subcommand_matches(SUB_COMMAND_SIGN_IN).is_some() {
        eprintln!("signin {:?} {:?}", server_address, auth_token_option);
        signin_command(server_address, auth_token_option.map(|s| s.to_string()));
    } else if matches.subcommand_matches(SUB_COMMAND_INFO).is_some() {
        eprintln!("info {:?} {:?}", server_address, auth_token_option);
        server_info(server_address, auth_token_option);
    } else if let Some(command_matches) = matches.subcommand_matches(SUB_COMMAND_SSO) {
        let user_id = command_matches.value_of(USER_ID_ARG).unwrap();
        let password = command_matches.value_of(PASSWORD_ARG).unwrap();
        eprintln!("sso {:?} {:?} {:?}", server_address, user_id, password);
        sso_command(server_address, user_id, password);
    } else if let Some(command_matches) = matches.subcommand_matches(SUB_COMMAND_CHECK) {
        let document_file_name = command_matches.value_of(DOCUMENT_ARG).unwrap();
        eprintln!("sso {:?} {:?} {:?}", server_address, document_file_name, auth_token_option);
        check(server_address, document_file_name, auth_token_option);
    }
}
