//! Theme definitions for consistent CLI styling

use console::{Style, Term};
use owo_colors::OwoColorize;

/// The DX CLI theme
#[allow(dead_code)]
pub struct Theme {
    term: Term,
    pub primary: Style,
    pub secondary: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub dim: Style,
    pub bold: Style,
    pub highlight: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl Theme {
    pub fn new() -> Self {
        Self {
            term: Term::stderr(),
            primary: Style::new().cyan(),
            secondary: Style::new().magenta(),
            success: Style::new().green(),
            warning: Style::new().yellow(),
            error: Style::new().red(),
            dim: Style::new().dim(),
            bold: Style::new().bold(),
            highlight: Style::new().cyan().bold(),
        }
    }

    /// Print the DX header/logo
    pub fn print_header(&self) {
        let version = env!("CARGO_PKG_VERSION");

        eprintln!();
        eprintln!(
            "  {}  {}",
            "▲".cyan().bold(),
            format!("DX v{version}").bright_white().bold()
        );
        eprintln!();
    }

    /// Print a section header
    pub fn print_section(&self, title: &str) {
        eprintln!(
            "  {} {}",
            "│".bright_black(),
            title.bright_white().bold()
        );
    }

    /// Print an info line
    pub fn print_info(&self, label: &str, value: &str) {
        eprintln!(
            "  {} {}: {}",
            "│".bright_black(),
            label.bright_black(),
            value.white()
        );
    }

    /// Print a success message
    pub fn print_success(&self, message: &str) {
        eprintln!();
        eprintln!("  {} {}", "✓".green().bold(), message.white());
    }

    /// Print a warning message
    pub fn print_warning(&self, message: &str) {
        eprintln!("  {} {}", "⚠".yellow().bold(), message.yellow());
    }

    /// Print an error message
    pub fn print_error(&self, message: &str) {
        eprintln!();
        eprintln!("  {} {}", "✕".red().bold(), message.red());
    }

    /// Print a step in a process
    #[allow(dead_code)]
    pub fn print_step(&self, step: usize, total: usize, message: &str) {
        eprintln!(
            "  {} {} {}",
            format!("[{step}/{total}]").bright_black(),
            "→".cyan(),
            message.white()
        );
    }

    /// Print a command hint
    pub fn print_hint(&self, command: &str) {
        eprintln!();
        eprintln!(
            "  {} Run {} to get started",
            "→".cyan(),
            format!("`{command}`").cyan().bold()
        );
    }

    /// Print a link
    pub fn print_link(&self, label: &str, url: &str) {
        eprintln!(
            "  {} {}: {}",
            "│".bright_black(),
            label.bright_black(),
            url.cyan().underline()
        );
    }

    /// Print a divider line
    pub fn print_divider(&self) {
        eprintln!("  {}", "─".repeat(48).bright_black());
    }

    /// Print an empty line with the sidebar
    #[allow(dead_code)]
    pub fn print_empty(&self) {
        eprintln!("  {}", "│".bright_black());
    }

    /// Print ready message for dev server
    pub fn print_ready(&self, url: &str, time_ms: u64) {
        eprintln!();
        eprintln!(
            "  {} Ready in {}",
            "✓".green().bold(),
            format!("{time_ms}ms").cyan().bold()
        );
        eprintln!();
        eprintln!(
            "  {} Local:   {}",
            "→".cyan(),
            url.cyan().bold().underline()
        );
        eprintln!();
    }

    /// Print cancelled message
    pub fn print_cancelled(&self) {
        eprintln!();
        eprintln!("  {} Cancelled", "○".bright_black());
        eprintln!();
    }

    /// Print build stats
    pub fn print_build_stats(&self, duration_ms: u64, bundle_size: &str, files: usize) {
        eprintln!();
        self.print_divider();
        eprintln!(
            "  {} Built in {} │ {} │ {} files",
            "✓".green().bold(),
            format!("{duration_ms}ms").cyan().bold(),
            bundle_size.magenta().bold(),
            files.to_string().white().bold()
        );
        self.print_divider();
        eprintln!();
    }

    /// Print test results
    pub fn print_test_results(
        &self,
        passed: usize,
        failed: usize,
        skipped: usize,
        duration_ms: u64,
    ) {
        eprintln!();
        self.print_divider();

        let status = if failed == 0 {
            "PASS".green().bold().to_string()
        } else {
            "FAIL".red().bold().to_string()
        };

        let failed_str = if failed > 0 {
            failed.to_string().red().bold().to_string()
        } else {
            failed.to_string().bright_black().to_string()
        };

        eprintln!(
            "  {} {} {} passed {} {} failed {} {} skipped {} in {}",
            status,
            "│".bright_black(),
            passed.to_string().green().bold(),
            "│".bright_black(),
            failed_str,
            "│".bright_black(),
            skipped.to_string().bright_black(),
            "│".bright_black(),
            format!("{duration_ms}ms").cyan()
        );

        self.print_divider();
        eprintln!();
    }

    /// Get terminal width
    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.term.size().1 as usize
    }

    /// Clear the terminal
    #[allow(dead_code)]
    pub fn clear(&self) {
        let _ = self.term.clear_screen();
    }
}

/// ASCII art logo for splash screens (kept minimal)
#[allow(dead_code)]
pub const LOGO_SMALL: &str = r"
   ██████╗ ██╗  ██╗
   ██╔══██╗╚██╗██╔╝
   ██║  ██║ ╚███╔╝ 
   ██║  ██║ ██╔██╗ 
   ██████╔╝██╔╝ ██╗
   ╚═════╝ ╚═╝  ╚═╝
";
