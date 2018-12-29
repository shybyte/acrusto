use std::env;

use clap::{App, Arg, SubCommand};
use clap::crate_version;
use lazy_static::lazy_static;
use log::{Level};
use simple_logger;

use crate::commands::capabilities::show_capabilities;
use crate::commands::check::check;
use crate::commands::info::server_info;
use crate::commands::signin::signin_command;
use crate::config::Config;
use crate::commands::common::CommonCommandConfig;
use crate::commands::check::CheckCommandOpts;

mod config;
mod api;
mod commands;
mod utils;

static SERVER_ADDRESS_ARG: &str = "acrolinx-address";
static ACCESS_TOKEN_ARG: &str = "access-token";
static SILENT_FLAG: &str = "silent";
static LOG_FLAG: &str = "log";
static OPEN_URL_FLAG: &str = "open";

static GUIDANCE_PROFILE_ARG: &str = "guidance-profile";
static FILES_ARG: &str = "files";

lazy_static! {
    static ref SERVER_ADDRESS_ENV_VAR: String = arg_name_to_env_var(SERVER_ADDRESS_ARG);
    static ref ACCESS_TOKEN_ENV_VAR: String = arg_name_to_env_var(ACCESS_TOKEN_ARG);
    static ref SILENT_ENV_VAR: String = arg_name_to_env_var(SILENT_FLAG);
    static ref LOG_ENV_VAR: String = arg_name_to_env_var(LOG_FLAG);
    static ref OPEN_URL_ENV_VAR: String = arg_name_to_env_var(OPEN_URL_FLAG);

    static ref GUIDANCE_PROFILE_ENV_VAR: String = arg_name_to_env_var(GUIDANCE_PROFILE_ARG);
    static ref FILES_ARG_ENV_VAR: String = arg_name_to_env_var(FILES_ARG);
}

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

    let silent_flag = create_arg(SILENT_FLAG, &OPEN_URL_ENV_VAR, &None)
        .short("s")
        .help("Restricts the console output to a minimum for scripting.")
        .takes_value(false);

    let log_flag = create_arg(LOG_FLAG, &LOG_ENV_VAR, &None)
        .help("Logs more than you want.")
        .takes_value(false);

    let open_url_flag = create_arg(OPEN_URL_FLAG, &SILENT_ENV_VAR, &None)
        .short("o")
        .help("Opens interactive sites, like the Dashboard, Sign-in page, and Scorecard.")
        .takes_value(false);

    let guidance_profile_arg = create_arg(GUIDANCE_PROFILE_ARG, &GUIDANCE_PROFILE_ENV_VAR, &None)
        .short("i") // TODO: Why i?
        .help("Sets the guidance profile. See capabilities for available options.");

    let files_arg = create_arg(FILES_ARG, &FILES_ARG_ENV_VAR, &None)
        .short("f")
        .multiple(true)
        .required(true)
        .help(" Sets the relative or absolute path to the file(s) to be checked.");

    let mut command_line_parser = App::new("acrusto")
        .version(crate_version!())
        .author("Marco Stahl <shybyte@gmail.com>")
        .about("Unofficial commandline tool for the Acrolinx Platform API")
        .arg(server_address_arg)
        .arg(auth_token_arg)
        .arg(silent_flag)
        .arg(log_flag)
        .arg(open_url_flag)
        .subcommand(SubCommand::with_name(SUB_COMMAND_SIGN_IN)
            .about("Signs in to Acrolinx via the Sign-in page and gets an access token."))
        .subcommand(SubCommand::with_name(SUB_COMMAND_INFO)
            .about("Shows the Acrolinx Platform version and information."))
        .subcommand(SubCommand::with_name(SUB_COMMAND_CAPABILITIES)
            .about("Lists the available check settings."))
        .subcommand(SubCommand::with_name(SUB_COMMAND_CHECK)
            .about("Checks the given file(s) with Acrolinx.")
            .arg(guidance_profile_arg)
            .arg(files_arg)
        );

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        command_line_parser.print_help().ok();
    }

    let matches = command_line_parser.get_matches();
    let access_token_option = matches.value_of(ACCESS_TOKEN_ARG);

    let command_config = CommonCommandConfig {
        acrolinx_address: matches.value_of(SERVER_ADDRESS_ARG).unwrap().to_string(),
        access_token: access_token_option.map(String::from),
        silent: matches.is_present(SILENT_FLAG),
        open_url: matches.is_present(OPEN_URL_FLAG),
    };

    if matches.is_present(LOG_FLAG) {
        simple_logger::init_with_level(Level::Info).ok();
    }

    if matches.subcommand_matches(SUB_COMMAND_SIGN_IN).is_some() {
        signin_command(&command_config);
    } else if matches.subcommand_matches(SUB_COMMAND_INFO).is_some() {
        server_info(&command_config);
    } else if matches.subcommand_matches(SUB_COMMAND_CAPABILITIES).is_some() {
        show_capabilities(&command_config);
    } else if let Some(command_matches) = matches.subcommand_matches(SUB_COMMAND_CHECK) {
        check(&command_config, &CheckCommandOpts {
            files: command_matches.values_of(FILES_ARG).unwrap().map(String::from).collect(),
            guidance_profile: command_matches.value_of(GUIDANCE_PROFILE_ARG).map(String::from)
        });
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