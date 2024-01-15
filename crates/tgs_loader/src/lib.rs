use crossterm::style::Color;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tgs_colors::custom;

use indicatif::{ProgressBar, ProgressStyle};

fn color_to_ansi(color: Color) -> String {
    match color {
        Color::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
        // Handle other color cases if needed
        _ => String::new(), // Default case
    }
}

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
    pub fn start(&self, prompt: &str) {
        let ansi_color = color_to_ansi(self.color);
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 ", "👌"])
                .template(&format!("{}{{spinner}} {{msg}}", ansi_color))
                .expect("Error message"),
        );
        let message = format!("Generating script for: {:?}", prompt);
        pb.set_message(message);
        thread::sleep(Duration::from_secs(5));
        pb.finish_with_message("Found something...");
    }

    /// `stop()` stops the loading indicator.
    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}
