use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use log::info;
use simple_logger;

use crate::api::checking::{CheckRequest, DocumentInfo};
use crate::api::checking::CheckOptions;
use crate::api::checking::AggregatedReportType;
use crate::api::common_types::ApiPollResponse;
use crate::commands::common::CommonCommandConfig;
use crate::api::checking::GuidanceProfileId;
use crate::api::AcroApi;
use crate::commands::common::connect_and_signin;
use ansi_term::Colour::{Red, Yellow, Green};
use ansi_term::ANSIGenericString;
use crate::api::checking::CheckResultQuality;
use crate::api::checking::QualityStatus;
use uuid::Uuid;
use crate::utils::open_url;
use glob::glob;
use regex::Regex;
use threadpool::ThreadPool;
use std::sync::Arc;
use crate::commands::check::progress::MultiProgressReporter;

use indicatif::ProgressBar;


mod progress;

pub struct CheckCommandOpts {
    pub files: Vec<String>,
    pub guidance_profile: Option<GuidanceProfileId>,
    pub max_concurrent: usize,
    pub auth_links: bool,
}

pub fn check(config: &CommonCommandConfig, opts: &CheckCommandOpts) {
    let api = Arc::new(connect_and_signin(&config).api);

    let reference_pattern = api.get_checking_capabilities().unwrap().referencePattern;
    let reference_regex = Regex::new(&reference_pattern).unwrap();

    let batch_id = format!("gen.acrusto.{}", Uuid::new_v4());

    let check_options = Arc::new(CheckOptions {
        guidanceProfileId: opts.guidance_profile.to_owned(),
        batchId: Some(batch_id.clone()),
    });

    println!("Generated batch id: {}", batch_id);

    let pool = ThreadPool::new(opts.max_concurrent);
    let multi_progress = Arc::new(MultiProgressReporter::new());

    for file_pattern in &opts.files {
        let filtered_files = glob(file_pattern).unwrap()
            .filter_map(Result::ok)
            .map(|path| { path.to_string_lossy().to_string() })
            .filter(|file| reference_regex.is_match(file));

        for path in filtered_files {
            let api = api.clone();
            let check_options = check_options.clone();
            let multi_progress = multi_progress.clone();

            pool.execute(move || {
                let pb = multi_progress.add(&path);

                let quality = check_file(&api, &check_options, &path, &pb);
                pb.finish_with_message(&format!("{}", colored_score(&quality)));
            });
        }
    }

    multi_progress.join();
    pool.join();

    show_aggregated_report(&config,opts, &api, &batch_id);
}

pub fn check_file<>(api: &AcroApi, check_options: &CheckOptions, filename: &str, progress_bar: &ProgressBar) -> CheckResultQuality {
    let mut f = File::open(filename).expect("File not found");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("Problem reading document");

//    println!("Start check {}", filename);text

    let check_request = CheckRequest {
        content: file_content,
        checkOptions: check_options.clone(),
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
                if let Some(percent) = p.progress.percent {
                    progress_bar.set_position(percent.round() as u64)
                }
                thread::sleep(Duration::from_secs(p.progress.retryAfter));
            }
        }
    }

//    println!("Check done for: {} {}", filename, colored_score(&check_result.quality));
    check_result.quality
}

fn colored_score(quality: &CheckResultQuality) -> ANSIGenericString<str> {
    let color = match quality.status {
        QualityStatus::red => Red,
        QualityStatus::yellow => Yellow,
        QualityStatus::green => Green,
    };

    color.paint(format!("{}", quality.score))
}

fn show_aggregated_report(config: &CommonCommandConfig, opts: &CheckCommandOpts, api: &AcroApi,
                          batch_id: &str) {
    let aggregated_report_links = api.get_link_to_aggregated_report(&batch_id).unwrap();
    info!("report_links = {:?}", aggregated_report_links);

    let report_type = match opts.auth_links {
        true => AggregatedReportType::shortWithApiKey,
        false => AggregatedReportType::shortWithoutApiKey
    };

    let aggregated_report_link = aggregated_report_links.reports.iter()
        .find(|report| report.reportType == report_type).unwrap();

    if !config.silent {
        println!("Find the Content Analysis Dashboard for your files here:")
    }
    println!("{} ", aggregated_report_link.link);

    if config.open_url {
        open_url(&aggregated_report_link.link).unwrap();
    }
}