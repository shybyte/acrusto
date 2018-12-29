use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use log::{info};
use simple_logger;

use crate::api::checking::{CheckRequest, DocumentInfo};
use crate::api::checking::CheckOptions;
use crate::api::common_types::ApiPollResponse;
use crate::commands::common::connect;
use crate::commands::common::CommandConfig;

pub fn check(config: CommandConfig, filename: &str) {
    let api = connect(&config);
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
