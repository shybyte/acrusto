use log::info;

use crate::api::checking::CheckingCapabilities;
use crate::commands::common::connect_and_signin;
use crate::commands::common::CommandConfig;
use crate::api::checking::GuidanceProfile;
use std::fmt::Debug;

pub fn show_capabilities(config: CommandConfig) {
    let api = connect_and_signin(&config);
    info!("{:?}", api.server_info());
    let capabilities = api.get_checking_capabilities().unwrap();
    if config.silent {
        println!("{}", serde_json::to_string_pretty(&capabilities).unwrap());
    } else {
        print_capabilities_for_humans(capabilities);
    }
}

fn print_capabilities_for_humans(capabilities: CheckingCapabilities) {
    print_section("GUIDANCE PROFILES", &capabilities.guidanceProfiles, format_guidance_profile);
    print_section("CONTENT FORMATS", &capabilities.contentFormats, |f| f.id.clone());
    print_section_enums("CONTENT ENCODINGS", &capabilities.contentEncodings);
    print_section_enums("CHECK TYPES", &capabilities.checkTypes);
    print_section_enums("REPORT TYPES", &capabilities.reportTypes);

    print_header("REFERENCE PATTERN");
    println!("{}", capabilities.referencePattern);
}

fn format_guidance_profile(guidance_profile: &GuidanceProfile) -> String {
    format!("{} ({})", guidance_profile.id, guidance_profile.displayName)
}

fn print_section<T, F>(header: &str, slice: &[T], f: F) where
    F: Fn(&T) -> String {
    print_header(header);
    for x in slice {
        println!("{}", f(x))
    }
}

fn print_section_enums<T: Debug>(header: &str, slice: &[T]) {
    print_section(header, slice, |e| format!("{:?}", e));
}

fn print_header(text: &str) {
    println!();
    println!("{}:", text.to_uppercase());
    println!("{}", "-".repeat(text.chars().count() + 1))
}