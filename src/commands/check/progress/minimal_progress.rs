use crate::api::checking::CheckResultQuality;
use crate::commands::check::progress::ProgressReporter;
use crate::commands::check::progress::MultiProgressReporter;

pub struct MinimalProgressReporter {
    path: String
}

impl ProgressReporter for MinimalProgressReporter {
    fn set_progress(&self, _percent: f64) {}

    fn finish(&self, quality: &CheckResultQuality) {
        println!("Check done for: {} {}", self.path, quality.score);
    }
}

pub struct MinimalMultiProgressReporter {}

impl MultiProgressReporter for MinimalMultiProgressReporter {
    fn add(&self, path: &str) -> Box<ProgressReporter> {
        Box::new(MinimalProgressReporter { path: path.to_string() })
    }
}
