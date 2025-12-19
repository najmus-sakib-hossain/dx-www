//! Spinner and loading indicators

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;

/// A modern spinner for async operations
pub struct Spinner {
    pb: ProgressBar,
    #[allow(dead_code)]
    message: String,
}

impl Spinner {
    /// Create a new spinner with a message
    #[allow(dead_code)]
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();

        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["   ◐", "   ◓", "   ◑", "   ◒"])
                .template("  {spinner:.cyan} {msg}")
                .unwrap(),
        );

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self {
            pb,
            message: message.to_string(),
        }
    }

    /// Create a spinner with dots style
    pub fn dots(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();

        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&[
                    "   ⠋", "   ⠙", "   ⠹", "   ⠸", "   ⠼", "   ⠴", "   ⠦", "   ⠧", "   ⠇",
                    "   ⠏",
                ])
                .template("  {spinner:.cyan} {msg}")
                .unwrap(),
        );

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self {
            pb,
            message: message.to_string(),
        }
    }

    /// Update the spinner message
    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    /// Mark spinner as successful
    pub fn success(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!("  {} {}", "✓".green().bold(), message.white());
    }

    /// Mark spinner as failed
    #[allow(dead_code)]
    pub fn error(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!("  {} {}", "✕".red().bold(), message.red());
    }

    /// Mark spinner as warning
    pub fn warn(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!("  {} {}", "⚠".yellow().bold(), message.yellow());
    }

    /// Finish the spinner without a status
    pub fn finish(self) {
        self.pb.finish_and_clear();
    }

    /// Stop the spinner without clearing (for intermediate output)
    #[allow(dead_code)]
    pub fn suspend<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.pb.suspend(f)
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.pb.finish_and_clear();
    }
}

/// Multi-spinner for parallel operations
#[allow(dead_code)]
pub struct MultiSpinner {
    spinners: Vec<Spinner>,
}

#[allow(dead_code)]
impl MultiSpinner {
    pub fn new() -> Self {
        Self {
            spinners: Vec::new(),
        }
    }

    pub fn add(&mut self, message: &str) -> usize {
        let spinner = Spinner::dots(message);
        self.spinners.push(spinner);
        self.spinners.len() - 1
    }

    pub fn finish_one(&mut self, index: usize, message: &str) {
        if let Some(spinner) = self.spinners.get(index) {
            spinner.set_message(message);
        }
    }
}

impl Default for MultiSpinner {
    fn default() -> Self {
        Self::new()
    }
}
