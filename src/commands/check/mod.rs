use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use glob::glob;
use log::info;
use regex::Regex;
use simple_logger;
use threadpool::ThreadPool;
use uuid::Uuid;

use crate::api::AcroApi;
use crate::api::checking::{CheckRequest, DocumentInfo};
use crate::api::checking::AggregatedReportType::{shortWithApiKey, shortWithoutApiKey};
use crate::api::checking::CheckOptions;
use crate::api::checking::CheckResultQuality;
use crate::api::checking::GuidanceProfileId;
use crate::api::common_types::ApiPollResponse;
use crate::api::errors::ApiError;
use crate::commands::check::progress::create_multi_progress_reporter;
use crate::commands::check::progress::ProgressReporter;
use crate::commands::common::CommonCommandConfig;
use crate::commands::common::connect_and_signin;
use crate::utils::open_url;
use crate::api::errors::CHECK_CANCELLED_ERROR;

mod progress;

pub struct CheckCommandOpts {
    pub files: Vec<String>,
    pub guidance_profile: Option<GuidanceProfileId>,
    pub max_concurrent: usize,
    pub auth_links: bool,
}

pub fn check(config: &CommonCommandConfig, opts: &CheckCommandOpts) {
    // Setup Ctrl-C handler.
    let stop_requested = Arc::new(AtomicBool::new(false));
    let stop_requested_for_handler = stop_requested.clone();
    ctrlc::set_handler(move || { stop_requested_for_handler.store(true, Ordering::SeqCst) }).expect("Error setting Ctrl-C handler");

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
        let found_files = glob(file_pattern).unwrap()
            .filter_map(Result::ok)
            .map(|path| { path.to_string_lossy().to_string() });

        for path in found_files {
            if stop_requested.load(Ordering::SeqCst) {
                return;
            }

            if !reference_regex.is_match(&path) {
                continue
            }

            let api = api.clone();
            let check_options = check_options.clone();
            let multi_progress = multi_progress.clone();
            let stop_requested = stop_requested.clone();

            pool.execute(move || {
                if stop_requested.load(Ordering::SeqCst) {
                    return;
                }
                let progress_reporter = multi_progress.add(&path);
                let result = check_file(&api, &check_options, &path,
                                        progress_reporter.as_ref(), stop_requested);
                progress_reporter.finish(&result);
            });
        }
    }

    multi_progress.join();
    pool.join();

    show_aggregated_report(&config, opts, &api, &batch_id);
}

pub fn check_file(api: &AcroApi, check_options: &CheckOptions, filename: &str,
                  progress_reporter: &ProgressReporter,
                  stop_requested: Arc<AtomicBool>) -> Result<CheckResultQuality, ApiError> {
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
        if stop_requested.load(Ordering::SeqCst) {
             api.cancel_check(&check.links)?;
             return Err(CHECK_CANCELLED_ERROR.clone());
        }

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