use colored::*;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tgs_colors::custom;

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
        let is_running = self.is_running.clone();
        *is_running.lock().unwrap() = true;
        let color_code = self.color;

        thread::spawn(move || {
            let spinner_chars = vec!['|', '/', '-', '\\'];
            let mut index = 0;

            println!("{:?}", color_code); // Sets color.
            while *is_running.lock().unwrap() {
                print!("\r{}", spinner_chars[index]);
                io::stdout().flush().unwrap();

                index = (index + 1) % spinner_chars.len();

                thread::sleep(Duration::from_millis(100));
            }

            println!();
            io::stdout().flush().unwrap();
        });
    }

    /// `stop()` stops the loading indicator.
    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}
