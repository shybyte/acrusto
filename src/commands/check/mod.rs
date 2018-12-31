use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use log::info;
use simple_logger;

use crate::api::checking::{CheckRequest, DocumentInfo};
use crate::api::checking::CheckOptions;
use crate::api::checking::AggregatedReportType::{shortWithApiKey, shortWithoutApiKey};
use crate::api::common_types::ApiPollResponse;
use crate::commands::common::CommonCommandConfig;
use crate::api::checking::GuidanceProfileId;
use crate::api::AcroApi;
use crate::commands::common::connect_and_signin;
use uuid::Uuid;
use crate::utils::open_url;
use glob::glob;
use regex::Regex;
use threadpool::ThreadPool;
use std::sync::Arc;
use crate::commands::check::progress::ProgressReporter;
use crate::commands::check::progress::create_multi_progress_reporter;
use crate::api::checking::CheckResultQuality;
use crate::api::errors::ApiError;

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
    let multi_progress = create_multi_progress_reporter();

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
                let progress_reporter = multi_progress.add(&path);
                let result = check_file(&api, &check_options, &path, progress_reporter.as_ref());
                progress_reporter.finish(&result);
            });
        }
    }

    multi_progress.join();
    pool.join();

    show_aggregated_report(&config, opts, &api, &batch_id);
}

pub fn check_file<>(api: &AcroApi, check_options: &CheckOptions, filename: &str,
                    progress_reporter: &ProgressReporter) -> Result<CheckResultQuality, ApiError> {
    let mut f = File::open(filename)?;
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("Problem reading document");

    let check_request = CheckRequest {
        content: file_content,
        checkOptions: check_options.clone(),
        document: Some(DocumentInfo {
            reference: fs::canonicalize(filename).ok()
                .map(|path| path.to_string_lossy().into_owned())
        }),
    };
    let check = api.check(&check_request)?;

    let mut check_poll_response;
    let check_result;
    loop {
        check_poll_response = api.get_checking_result(&check.links)?;
        info!("check_poll_response = {:?}", check_poll_response);
        match check_poll_response {
            ApiPollResponse::SuccessResponse(s) => {
                check_result = s.data;
                break;
            }
            ApiPollResponse::ProgressResponse(p) => {
                info!("progress = {:?}", p.progress.percent);
                if let Some(percent) = p.progress.percent {
                    progress_reporter.set_progress(percent);
                }
                thread::sleep(Duration::from_secs(p.progress.retryAfter));
            }
        }
    }

    Ok(check_result.quality)
}

fn show_aggregated_report(config: &CommonCommandConfig, opts: &CheckCommandOpts, api: &AcroApi,
                          batch_id: &str) {
    let aggregated_report_links = api.get_link_to_aggregated_report(&batch_id).unwrap();
    info!("report_links = {:?}", aggregated_report_links);

    let report_type = if opts.auth_links { shortWithApiKey } else { shortWithoutApiKey };

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