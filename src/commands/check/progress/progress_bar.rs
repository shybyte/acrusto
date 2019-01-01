use indicatif;
use console::Term;
use indicatif::ProgressStyle;
use std::thread::spawn;
use std::sync::Arc;
use crate::api::checking::CheckResultQuality;
use crate::api::checking::QualityStatus;
use ansi_term::Colour::{Red, Yellow, Green};
use ansi_term::ANSIGenericString;
use crate::commands::check::progress::ProgressReporter;
use crate::commands::check::progress::MultiProgressReporter;
use crate::api::errors::ApiError;
use crate::api::errors::CHECK_CANCELLED_ERROR_TYPE;

pub struct ProgressBarReporter {
    progress_bar: indicatif::ProgressBar,
}

impl ProgressBarReporter {
    fn new(progress_bar: indicatif::ProgressBar, path: &str) -> Self {
        let term = Term::stdout();
        let filename_width = (term.size().1 / 2).max(10);
        let progress_template = "{prefix:".to_string() +
            &filename_width.to_string() +
            "!} [{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>3}% {msg:>3}";
        let progress_style = ProgressStyle::default_bar()
            .template(&progress_template)
            .progress_chars("##-");

        progress_bar.set_style(progress_style);
        progress_bar.set_prefix(&path);
        progress_bar.enable_steady_tick(500);

        Self { progress_bar }
    }
}

impl ProgressReporter for ProgressBarReporter {
    fn set_progress(&self, percent: f64) {
        self.progress_bar.set_position(percent.round() as u64);
    }

    fn finish(&self, result: &Result<CheckResultQuality, ApiError>) {
        let message = match result {
            Ok(quality) => colored_score(quality),
            Err(error) =>
                if error._type == CHECK_CANCELLED_ERROR_TYPE {
                    Red.paint("CNL")
                } else {
                    Red.blink().paint("ERR")
                }
        };
        self.progress_bar.finish_with_message(&format!("{}", message));
    }
}

pub struct MultiProgressBarReporter {
    pub multi_progress: Arc<indicatif::MultiProgress>,
}

impl MultiProgressBarReporter {
    pub fn new() -> Self {
        let multi_progress = Arc::new(indicatif::MultiProgress::new());

        {
            // Workaround for missing progress bar rendering if only one progress is added.
            let multi_progress = multi_progress.clone();

            spawn(move || {
                let pb = multi_progress.add(indicatif::ProgressBar::new(100));
                pb.inc(1);
                pb.finish_and_clear();
            });
        }

        Self { multi_progress }
    }
}

impl MultiProgressReporter for MultiProgressBarReporter {
    fn add(&self, path: &str) -> Box<ProgressReporter> {
        let progress_bar = self.multi_progress.add(indicatif::ProgressBar::new(100));
        Box::new(ProgressBarReporter::new(progress_bar, path))
    }

    fn join(&self) {
        self.multi_progress.join().ok();
    }
}

fn colored_score(quality: &CheckResultQuality) -> ANSIGenericString<str> {
    let color = match quality.status {
        QualityStatus::red => Red,
        QualityStatus::yellow => Yellow,
        QualityStatus::green => Green,
    };

    color.paint(format!("{}", quality.score))
}