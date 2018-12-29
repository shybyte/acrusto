use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use log::info;
use simple_logger;

use crate::api::checking::{CheckRequest, DocumentInfo};
use crate::api::checking::CheckOptions;
use crate::api::common_types::ApiPollResponse;
use crate::commands::common::CommonCommandConfig;
use crate::api::checking::GuidanceProfileId;
use crate::api::AcroApi;
use crate::commands::common::connect_and_signin;
use ansi_term::Colour::{Red, Yellow, Green};
use ansi_term::ANSIGenericString;
use crate::api::checking::CheckResultQuality;
use crate::api::checking::QualityStatus;

pub struct CheckCommandOpts {
    pub files: Vec<String>,
    pub guidance_profile: Option<GuidanceProfileId>,
}

pub fn check(config: CommonCommandConfig, opts: &CheckCommandOpts) {
    let api = connect_and_signin(&config).api;

    for file in &opts.files {
        check_file(&api, &opts.guidance_profile, &file);
    }
}

pub fn check_file(api: &AcroApi, guidance_profile: &Option<GuidanceProfileId>, filename: &str) {
    let mut f = File::open(filename).expect("File not found");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("Problem reading document");
    info!("file_content = {:?}", file_content);

    let check_request = CheckRequest {
        content: file_content,
        checkOptions: CheckOptions { guidanceProfileId: guidance_profile.to_owned() },
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
    println!("Check done for: {} {}", filename, colored_score(&check_result.quality));
}

fn colored_score(quality: &CheckResultQuality) -> ANSIGenericString<str> {
    let color = match quality.status {
        QualityStatus::red => Red,
        QualityStatus::yellow => Yellow,
        QualityStatus::green => Green,
    };

    color.paint(format!("{}", quality.score))
}