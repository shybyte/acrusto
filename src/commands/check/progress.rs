use indicatif::MultiProgress;
use indicatif::ProgressBar;
use console::Term;
use indicatif::ProgressStyle;
use std::thread::spawn;
use std::sync::Arc;

pub struct ProgressReporter {
    progress_bar: ProgressBar,
}

impl ProgressReporter {
    fn new() -> Self {
        ProgressReporter {
            progress_bar: ProgressBar::new(100)
        }
    }
}

pub struct MultiProgressReporter {
    pub multi_progress: Arc<MultiProgress>,
}

impl MultiProgressReporter {
    pub fn new() -> Self {
        let multi_progress = Arc::new(MultiProgress::new());

        {
            // Workaround for missing progress bar rendering if only one progress is added.
            let multi_progress = multi_progress.clone();

            spawn(move || {
                let pb = multi_progress.add(ProgressBar::new(100));
                pb.inc(1);
                pb.finish_and_clear();
            });
        }

        MultiProgressReporter {
            multi_progress
        }
    }

    pub fn join(&self) {
        self.multi_progress.join().ok();
    }

    pub fn add(&self, path: &str) -> ProgressBar {
        let term = Term::stdout();
        let filename_width = (term.size().1 / 2).max(10);
        let progress_template = "{prefix:".to_string() + &filename_width.to_string() + "!} [{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}% {msg:5}";
        let progress_style = ProgressStyle::default_bar()
            .template(&progress_template)
            .progress_chars("##-");

        let progress_bar = self.multi_progress.add(ProgressBar::new(100));
        progress_bar.set_style(progress_style);
        progress_bar.set_prefix(&path);
        progress_bar.enable_steady_tick(500);
        progress_bar.inc(1);

        progress_bar
    }
}
