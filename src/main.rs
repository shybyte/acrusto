use std::env;

use clap::{App, Arg, SubCommand};
use clap::crate_version;
use lazy_static::lazy_static;
use log::{info, Level};
use simple_logger;

use crate::commands::capabilities::show_capabilities;
use crate::commands::check::check;
use crate::commands::info::server_info;
use crate::commands::signin::signin_command;
use crate::config::Config;
use crate::commands::common::CommandConfig;

mod config;
mod api;
mod commands;

static SERVER_ADDRESS_ARG: &str = "acrolinx-address";
static ACCESS_TOKEN_ARG: &str = "access-token";
static SILENT_FLAG: &str = "silent";

lazy_static! {
    static ref SERVER_ADDRESS_ENV_VAR: String = arg_name_to_env_var(SERVER_ADDRESS_ARG);
    static ref ACCESS_TOKEN_ENV_VAR: String = arg_name_to_env_var(ACCESS_TOKEN_ARG);
    static ref SILENT_ENV_VAR: String = arg_name_to_env_var(SILENT_FLAG);
}

static DOCUMENT_ARG: &str = "DOCUMENT";

static SUB_COMMAND_SIGN_IN: &str = "signin";
static SUB_COMMAND_INFO: &str = "info";
static SUB_COMMAND_CAPABILITIES: &str = "capabilities";
static SUB_COMMAND_CHECK: &str = "check";

fn main() {
    let default_config = Config::read();

    let auth_token_arg = create_arg(ACCESS_TOKEN_ARG, &ACCESS_TOKEN_ENV_VAR, &default_config.access_token)
        .short("t")
        .help("Sets an access token to authenticate a user. We recommend setting the access token as an environment variable.")
        .takes_value(true);

    let server_address_arg = create_arg(SERVER_ADDRESS_ARG, &SERVER_ADDRESS_ENV_VAR, &default_config.acrolinx_address)
        .short("a")
        .required(true)
        .help("Sets the URL of the Acrolinx Platform.")
        .takes_value(true);

    let silent_flag = create_arg(SILENT_FLAG, &SILENT_ENV_VAR, &None)
        .short("s")
        .help("Restricts the console output to a minimum for scripting.")
        .takes_value(false);

    let mut command_line_parser = App::new("acrusto")
        .version(crate_version!())
        .author("Marco Stahl <shybyte@gmail.com>")
        .about("Unofficial commandline tool for the Acrolinx Platform API")
        .arg(server_address_arg)
        .arg(auth_token_arg)
        .arg(silent_flag)
        .subcommand(SubCommand::with_name(SUB_COMMAND_SIGN_IN)
            .about("Signs in to Acrolinx via the Sign-in page and gets an access token."))
        .subcommand(SubCommand::with_name(SUB_COMMAND_INFO)
            .about("Shows the Acrolinx Platform version and information."))
        .subcommand(SubCommand::with_name(SUB_COMMAND_CAPABILITIES)
            .about("Lists the available check settings."))
        .subcommand(SubCommand::with_name(SUB_COMMAND_CHECK)
            .about("Checks the given file(s) with Acrolinx.")
            .arg(Arg::with_name(DOCUMENT_ARG).required(true).index(1))
        );

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        command_line_parser.print_help().ok();
    }

    let matches = command_line_parser.get_matches();
    let auth_token_option = matches.value_of(ACCESS_TOKEN_ARG);
    let server_address = matches.value_of(SERVER_ADDRESS_ARG).unwrap();

    let command_config = CommandConfig {
        acrolinx_address: server_address.to_string(),
        access_token: auth_token_option.map(String::from),
        silent: matches.is_present(SILENT_FLAG)
    };

    if !command_config.silent {
        simple_logger::init_with_level(Level::Info).ok();
    }

    if matches.subcommand_matches(SUB_COMMAND_SIGN_IN).is_some() {
        info!("signin {:?} {:?}", server_address, auth_token_option);
        signin_command(command_config);
    } else if matches.subcommand_matches(SUB_COMMAND_INFO).is_some() {
        info!("info {:?} {:?}", server_address, auth_token_option);
        server_info(command_config);
    } else if matches.subcommand_matches(SUB_COMMAND_CAPABILITIES).is_some() {
        info!("show_capabilities {:?} {:?}", server_address, auth_token_option);
        show_capabilities(command_config);
    } else if let Some(command_matches) = matches.subcommand_matches(SUB_COMMAND_CHECK) {
        let document_file_name = command_matches.value_of(DOCUMENT_ARG).unwrap();
        info!("check {:?} {:?} {:?}", server_address, document_file_name, auth_token_option);
        check(command_config, document_file_name);
    }
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

fn arg_name_to_env_var(arg_name: &str) -> String {
    ("ACROLINX_".to_string() + arg_name).to_uppercase().replace("-", "_")
}