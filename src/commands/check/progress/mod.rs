use crate::api::checking::CheckResultQuality;
use crate::commands::check::progress::progress_bar::MultiProgressBarReporter;
use std::sync::Arc;
use console::Term;
use crate::commands::check::progress::minimal_progress::MinimalMultiProgressReporter;

mod progress_bar;
mod minimal_progress;

pub trait ProgressReporter {
    fn set_progress(&self, percent: f64);
    fn finish(&self, quality: &CheckResultQuality);
}

pub trait MultiProgressReporter: Sync + Send {
    fn add(&self, path: &str) -> Box<ProgressReporter>;
    fn join(&self) {}
}

pub fn create_multi_progress_reporter() -> Arc<MultiProgressReporter> {
    let term = Term::stdout();
    if term.is_term() {
        Arc::new(MultiProgressBarReporter::new())
    } else {
        Arc::new(MinimalMultiProgressReporter{})
    }
}