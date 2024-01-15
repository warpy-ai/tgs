use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tgs_colors::custom;

use indicatif::{ProgressBar, ProgressStyle};

/// A `LoadingIndicator` handles the animation of a loading indicator.
pub struct LoadingIndicator {
    /// A mutex to indicate whether the loading indicator is running or not.
    is_running: Arc<Mutex<bool>>,

    color: Color,
}

impl Default for LoadingIndicator {
    fn default() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            color: custom::DARK_WHITE,
        }
    }
}

impl LoadingIndicator {
    /// `new()` initializes a new loading indicator.
    pub fn new(color: Color) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            color,
        }
    }

    /// `start()` spawn a new thread and display the loading indicator.
    /// It ends when `stop()` is called.
    pub fn start(&self) {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        pb.set_message("Calculating...");
        thread::sleep(Duration::from_secs(5));
        pb.finish_with_message("Done");
    }

    /// `stop()` stops the loading indicator.
    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}
