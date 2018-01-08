#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

mod api;

use api::AcroApi;
use api::LoginRequestResponse::*;

fn main() {
    let api = AcroApi::new("https://test-latest-ssl.acrolinx.com");

    println!("Yeah, there is a server: {:?}", api.server_version());

    let login_response = api.login().unwrap();
    println!("login_response = {:?}", login_response);

    match login_response {
        LoginLinks(login_links_response) => {
            println!("Please signin at {:?}", login_links_response.links.interactive);
            api.wait_for_signin(&login_links_response.links).unwrap();
        },
        LoggedIn(logged_in) => {
            println!("You are already logged in as {:?}", logged_in.userId);
        }
    }
}
