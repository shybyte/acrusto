use crate::api::checking::CheckResultQuality;
use crate::commands::check::progress::ProgressReporter;
use crate::commands::check::progress::MultiProgressReporter;
use crate::api::errors::ApiError;
use crate::api::errors::CHECK_CANCELLED_ERROR_TYPE;

pub struct MinimalProgressReporter {
    path: String
}

impl ProgressReporter for MinimalProgressReporter {
    fn set_progress(&self, _percent: f64) {}

    fn finish(&self, result: &Result<CheckResultQuality, ApiError>) {
        match result {
            Ok(quality) => {
                println!("Check done for: {} {}", self.path, quality.score);
            }
            Err(error) => {
                if error._type == CHECK_CANCELLED_ERROR_TYPE {
                    println!("Check cancelled: {}", self.path);
                } else {
                    println!("Error in {}: {}({})", self.path, error.title, error.detail);
                }
            }
        }
    }
}

pub struct MinimalMultiProgressReporter {}

impl MultiProgressReporter for MinimalMultiProgressReporter {
    fn add(&self, path: &str) -> Box<ProgressReporter> {
        Box::new(MinimalProgressReporter { path: path.to_string() })
    }
}
