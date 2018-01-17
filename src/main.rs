#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate hyper;


mod api;

use std::env;
use clap::{Arg, App, SubCommand};
use api::AcroApi;
use api::LoginRequestResponse::*;

fn signin_command(server_address: &str, auth_token_option: Option<String>) {
    let api = AcroApi::new(server_address); // "https://test-latest-ssl.acrolinx.com"

    println!("Yeah, there is a server: {:?}", api.server_version());

    let login_response = api.login(auth_token_option).unwrap();
    println!("login_response = {:?}", login_response);

    match login_response {
        LoginLinks(login_links_response) => {
            println!("Please signin at {:?}", login_links_response.links.interactive);
            let logged_in = api.wait_for_signin(&login_links_response.links).unwrap();
            println!("authToken = {:?}", logged_in.authToken);
            println!("You are logged in as {:?}", logged_in.userId);
        }
        LoggedIn(logged_in) => {
            println!("You are already logged in as {:?}", logged_in.userId);
        }
    }
}

static SERVER_ADDRESS_ARG: &str = "SERVER_ADDRESS";
static AUTH_TOKEN_ARG: &str = "authToken";

fn main() {
    let mut command_line_parser = App::new("acrusto")
        .version(crate_version!())
        .author("Marco Stahl <shybyte@gmail.com>")
        .about("Unofficial commandline tool for the Acrolinx Platform API")
        .arg(Arg::with_name(AUTH_TOKEN_ARG)
            .short("a")
            .long("authToken")
            .help("Use an authToken")
            .takes_value(true))
        .subcommand(SubCommand::with_name("signin")
            .about("Signin to Acrolinx")
            .arg(Arg::with_name(SERVER_ADDRESS_ARG)
                .required(true)
                .index(1)
            )
        );

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        command_line_parser.print_help().ok();
    }

    let matches = command_line_parser.get_matches();

    if let Some(command_matches) = matches.subcommand_matches("signin") {
        let server_address = command_matches.value_of(SERVER_ADDRESS_ARG).unwrap();
        let auth_token_option = matches.value_of(AUTH_TOKEN_ARG);
        eprintln!("signin {:?} {:?}", server_address, auth_token_option);
        signin_command(server_address, auth_token_option.map(|s| s.to_string()));
    }
}
