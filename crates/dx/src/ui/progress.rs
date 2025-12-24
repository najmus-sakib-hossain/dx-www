//! Progress bars for long-running operations

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

/// A styled progress bar
pub struct Progress {
    pb: ProgressBar,
}

impl Progress {
    /// Create a new progress bar with a total count
    pub fn new(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);

        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {msg}\n  [{bar:40.cyan/bright_black}] {pos}/{len} {eta}")
                .unwrap()
                .progress_chars("━━─"),
        );

        pb.set_message(message.to_string());

        Self { pb }
    }

    /// Create a byte-based progress bar (for downloads)
    #[allow(dead_code)]
    pub fn bytes(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);

        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "  {msg}\n  [{bar:40.cyan/bright_black}] {bytes}/{total_bytes} ({bytes_per_sec})",
                )
                .unwrap()
                .progress_chars("━━─"),
        );

        pb.set_message(message.to_string());

        Self { pb }
    }

    /// Increment the progress
    #[allow(dead_code)]
    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }

    /// Set the current position
    pub fn set_position(&self, pos: u64) {
        self.pb.set_position(pos);
    }

    /// Update the message
    #[allow(dead_code)]
    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    /// Finish the progress bar with a success message
    pub fn finish_success(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!("  {} {}", "✓".green().bold(), message.white());
    }

    /// Finish the progress bar with an error
    #[allow(dead_code)]
    pub fn finish_error(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!("  {} {}", "✕".red().bold(), message.red());
    }

    /// Get the inner progress bar for advanced usage
    #[allow(dead_code)]
    pub fn inner(&self) -> &ProgressBar {
        &self.pb
    }
}

/// Progress tracker for multiple parallel operations
#[allow(dead_code)]
pub struct MultiProgressTracker {
    mp: MultiProgress,
    bars: Vec<ProgressBar>,
}

#[allow(dead_code)]
impl MultiProgressTracker {
    pub fn new() -> Self {
        Self {
            mp: MultiProgress::new(),
            bars: Vec::new(),
        }
    }

    /// Add a new progress bar
    pub fn add(&mut self, total: u64, message: &str) -> usize {
        let pb = self.mp.add(ProgressBar::new(total));

        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {prefix:.cyan} [{bar:30.cyan/bright_black}] {pos}/{len}")
                .unwrap()
                .progress_chars("━━─"),
        );

        pb.set_prefix(message.to_string());

        self.bars.push(pb);
        self.bars.len() - 1
    }

    /// Increment a specific progress bar
    pub fn inc(&self, index: usize, delta: u64) {
        if let Some(pb) = self.bars.get(index) {
            pb.inc(delta);
        }
    }

    /// Finish a specific progress bar
    pub fn finish(&self, index: usize) {
        if let Some(pb) = self.bars.get(index) {
            pb.finish();
        }
    }

    /// Finish all progress bars
    pub fn finish_all(self) {
        for pb in self.bars {
            pb.finish_and_clear();
        }
    }
}

impl Default for MultiProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}
