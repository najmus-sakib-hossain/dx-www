//! README analysis module
//!
//! Analyzes README.md files for completeness and content quality.
//! Detects required sections, badge formats, and inappropriate content.

use regex::Regex;
use std::fs;
use std::path::Path;

/// Analysis result for a README file
#[derive(Debug, Clone, Default)]
pub struct ReadmeAnalysis {
    /// Whether the README exists
    pub exists: bool,
    /// Whether the README has a title (H1 heading)
    pub has_title: bool,
    /// Whether the README has a description paragraph
    pub has_description: bool,
    /// Whether the README has badges
    pub has_badges: bool,
    /// Whether the README has an installation section
    pub has_installation: bool,
    /// Whether the README has a usage/examples section
    pub has_usage: bool,
    /// Whether the README has license information
    pub has_license: bool,
    /// Whether the README has a subcrate table (for parent crates)
    pub has_subcrate_table: bool,
    /// Whether the README contains task instructions (bad)
    pub contains_task_instructions: bool,
    /// Whether the README contains raw prompts (bad)
    pub contains_raw_prompts: bool,
    /// Whether the README contains development notes (bad)
    pub contains_dev_notes: bool,
    /// The detected badge format
    pub badge_format: Option<BadgeFormat>,
    /// Whether the file is empty or nearly empty
    pub is_empty: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BadgeFormat {
    ShieldsIo,
    BadgenNet,
    Custom,
    Inconsistent,
}

/// Analyze a README.md file in the given crate directory
pub fn analyze_readme(crate_dir: &Path) -> ReadmeAnalysis {
    let readme_path = crate_dir.join("README.md");
    
    if !readme_path.exists() {
        return ReadmeAnalysis {
            exists: false,
            ..Default::default()
        };
    }

    let content = match fs::read_to_string(&readme_path) {
        Ok(c) => c,
        Err(_) => return ReadmeAnalysis {
            exists: true,
            ..Default::default()
        },
    };

    analyze_readme_content(&content)
}

/// Analyze README content string
pub fn analyze_readme_content(content: &str) -> ReadmeAnalysis {
    let is_empty = content.trim().is_empty() || content.trim().len() < 50;
    
    ReadmeAnalysis {
        exists: true,
        has_title: detect_title(content),
        has_description: detect_description(content),
        has_badges: detect_badges(content),
        has_installation: detect_installation_section(content),
        has_usage: detect_usage_section(content),
        has_license: detect_license_section(content),
        has_subcrate_table: detect_subcrate_table(content),
        contains_task_instructions: detect_task_instructions(content),
        contains_raw_prompts: detect_raw_prompts(content),
        contains_dev_notes: detect_dev_notes(content),
        badge_format: detect_badge_format(content),
        is_empty,
    }
}

/// Detect if README has a title (H1 heading)
fn detect_title(content: &str) -> bool {
    let h1_pattern = Regex::new(r"^#\s+\S").unwrap();
    content.lines().any(|line| h1_pattern.is_match(line))
}

/// Detect if README has a description paragraph after the title
fn detect_description(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    
    // Look for a non-empty paragraph after the title
    let mut found_title = false;
    let mut in_description = false;
    
    for line in lines {
        if line.starts_with("# ") {
            found_title = true;
            continue;
        }
        
        if found_title {
            let trimmed = line.trim();
            // Skip empty lines and badges
            if trimmed.is_empty() || trimmed.starts_with("[![") || trimmed.starts_with("![") {
                continue;
            }
            // Skip other headings
            if trimmed.starts_with('#') {
                if in_description {
                    return true;
                }
                continue;
            }
            // Found description text
            if trimmed.len() > 20 {
                in_description = true;
            }
        }
    }
    
    in_description
}

/// Detect if README has badges
fn detect_badges(content: &str) -> bool {
    let badge_patterns = [
        r"\[!\[.*?\]\(.*?\)\]\(.*?\)",  // [![alt](img)](link) format
        r"!\[.*?badge.*?\]",             // ![badge] format
        r"shields\.io",                   // shields.io URLs
        r"badgen\.net",                   // badgen.net URLs
        r"img\.shields\.io",              // img.shields.io URLs
    ];
    
    badge_patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect badge format used
fn detect_badge_format(content: &str) -> Option<BadgeFormat> {
    let has_shields = content.contains("shields.io") || content.contains("img.shields.io");
    let has_badgen = content.contains("badgen.net");
    
    match (has_shields, has_badgen) {
        (true, true) => Some(BadgeFormat::Inconsistent),
        (true, false) => Some(BadgeFormat::ShieldsIo),
        (false, true) => Some(BadgeFormat::BadgenNet),
        (false, false) => {
            // Check for any badge-like patterns
            if detect_badges(content) {
                Some(BadgeFormat::Custom)
            } else {
                None
            }
        }
    }
}

/// Detect if README has an installation section
fn detect_installation_section(content: &str) -> bool {
    let patterns = [
        r"(?i)##?\s*installation",
        r"(?i)##?\s*getting\s*started",
        r"(?i)##?\s*setup",
        r"(?i)##?\s*quick\s*start",
        r"\[dependencies\]",  // Cargo.toml snippet
        r"cargo\s+add",       // cargo add command
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect if README has a usage/examples section
fn detect_usage_section(content: &str) -> bool {
    let patterns = [
        r"(?i)##?\s*usage",
        r"(?i)##?\s*examples?",
        r"(?i)##?\s*how\s*to\s*use",
        r"(?i)##?\s*api",
        r"```rust",           // Rust code examples
        r"```toml",           // TOML examples
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect if README has license information
fn detect_license_section(content: &str) -> bool {
    let patterns = [
        r"(?i)##?\s*license",
        r"(?i)MIT\s*(OR|/)\s*Apache",
        r"(?i)licensed\s+under",
        r"(?i)LICENSE",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect if README has a subcrate table
fn detect_subcrate_table(content: &str) -> bool {
    let patterns = [
        r"\|.*\|.*\|",        // Markdown table
        r"(?i)subcrates?",
        r"(?i)modules?.*table",
        r"(?i)components?.*list",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect task instructions in README (inappropriate content)
fn detect_task_instructions(content: &str) -> bool {
    let patterns = [
        r"(?i)please\s+(read|create|implement|write|add)",
        r"(?i)step\s+\d+:",
        r"(?i)task\s*\d*:",
        r"(?i)todo:",
        r"(?i)fixme:",
        r"(?i)your\s+task\s+is",
        r"(?i)you\s+should",
        r"(?i)you\s+need\s+to",
        r"(?i)implement\s+the\s+following",
        r"(?i)create\s+a\s+tasklist",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect raw AI prompts in README (inappropriate content)
fn detect_raw_prompts(content: &str) -> bool {
    let patterns = [
        r"(?i)<prompt>",
        r"(?i)</prompt>",
        r"(?i)```prompt",
        r"(?i)as\s+an?\s+(ai|assistant|llm)",
        r"(?i)you\s+are\s+an?\s+(ai|assistant)",
        r"(?i)generate\s+(code|documentation)",
        r"(?i)write\s+me\s+a",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

/// Detect development notes in README (should be moved)
fn detect_dev_notes(content: &str) -> bool {
    let patterns = [
        r"(?i)##?\s*development\s*notes?",
        r"(?i)##?\s*internal\s*notes?",
        r"(?i)##?\s*wip",
        r"(?i)##?\s*work\s*in\s*progress",
        r"(?i)##?\s*draft",
        r"(?i)\[wip\]",
        r"(?i)\[draft\]",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_title() {
        assert!(detect_title("# My Crate\n\nDescription"));
        assert!(detect_title("# dx-www\n"));
        assert!(!detect_title("## Not a title\n"));
        assert!(!detect_title("No title here"));
    }

    #[test]
    fn test_detect_badges() {
        assert!(detect_badges("[![Crates.io](https://img.shields.io/crates/v/foo.svg)](https://crates.io/crates/foo)"));
        assert!(detect_badges("![badge](https://badgen.net/badge/foo/bar)"));
        assert!(!detect_badges("Just some text"));
    }

    #[test]
    fn test_detect_task_instructions() {
        assert!(detect_task_instructions("Please read the following"));
        assert!(detect_task_instructions("Step 1: Do something"));
        assert!(detect_task_instructions("TODO: implement this"));
        assert!(!detect_task_instructions("This is a normal description"));
    }

    #[test]
    fn test_detect_installation_section() {
        assert!(detect_installation_section("## Installation\n\nRun cargo add"));
        assert!(detect_installation_section("[dependencies]\nfoo = \"1.0\""));
        assert!(!detect_installation_section("Just some text"));
    }

    #[test]
    fn test_analyze_empty_readme() {
        let analysis = analyze_readme_content("");
        assert!(analysis.is_empty);
        assert!(!analysis.has_title);
    }

    #[test]
    fn test_analyze_complete_readme() {
        let content = r#"# My Crate

[![Crates.io](https://img.shields.io/crates/v/my-crate.svg)](https://crates.io/crates/my-crate)

A great crate that does amazing things for developers.

## Installation

```toml
[dependencies]
my-crate = "1.0"
```

## Usage

```rust
use my_crate::foo;
```

## License

MIT OR Apache-2.0
"#;
        let analysis = analyze_readme_content(content);
        assert!(analysis.has_title);
        assert!(analysis.has_description);
        assert!(analysis.has_badges);
        assert!(analysis.has_installation);
        assert!(analysis.has_usage);
        assert!(analysis.has_license);
        assert!(!analysis.contains_task_instructions);
        assert!(!analysis.is_empty);
    }
}
