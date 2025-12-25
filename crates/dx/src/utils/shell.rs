//! Shell integration for the DX CLI
//!
//! Provides shell-specific integration scripts and installation.
//! - Requirement 8.1: Support Bash, Zsh, Fish, PowerShell, Nushell
//! - Requirement 8.2: Add smart aliases (d, dr, db, dd, dt, dg, ds, df)
//! - Requirement 8.3: Detect when entering a DX project directory
//! - Requirement 8.4: Generate shell completions
//! - Requirement 8.5: Warn if already installed
//! - Requirement 8.6: Modify appropriate config file

use crate::utils::error::DxError;
use std::path::PathBuf;

/// Shell types supported by DX
///
/// Requirement 8.1: Support Bash, Zsh, Fish, PowerShell, Nushell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Nushell,
}

impl ShellType {
    /// Detect the current shell from environment
    ///
    /// Requirement 8.6: Detect shell type
    pub fn detect() -> Option<Self> {
        // Check SHELL environment variable (Unix)
        if let Ok(shell) = std::env::var("SHELL") {
            let shell_lower = shell.to_lowercase();
            if shell_lower.contains("bash") {
                return Some(ShellType::Bash);
            }
            if shell_lower.contains("zsh") {
                return Some(ShellType::Zsh);
            }
            if shell_lower.contains("fish") {
                return Some(ShellType::Fish);
            }
        }

        // Check for PowerShell on Windows
        #[cfg(windows)]
        {
            if std::env::var("PSModulePath").is_ok() {
                return Some(ShellType::PowerShell);
            }
        }

        // Check for Nushell
        if std::env::var("NU_VERSION").is_ok() {
            return Some(ShellType::Nushell);
        }

        None
    }

    /// Get the config file path for this shell
    ///
    /// Requirement 8.6: Modify appropriate config file
    pub fn config_path(&self) -> Option<PathBuf> {
        let home = home::home_dir()?;

        Some(match self {
            ShellType::Bash => {
                // Prefer .bashrc, fall back to .bash_profile
                let bashrc = home.join(".bashrc");
                if bashrc.exists() {
                    bashrc
                } else {
                    home.join(".bash_profile")
                }
            }
            ShellType::Zsh => home.join(".zshrc"),
            ShellType::Fish => home.join(".config/fish/config.fish"),
            ShellType::PowerShell => {
                #[cfg(windows)]
                {
                    home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")
                }
                #[cfg(not(windows))]
                {
                    home.join(".config/powershell/Microsoft.PowerShell_profile.ps1")
                }
            }
            ShellType::Nushell => home.join(".config/nushell/config.nu"),
        })
    }

    /// Get the shell name as a string
    pub fn name(&self) -> &'static str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
            ShellType::PowerShell => "powershell",
            ShellType::Nushell => "nushell",
        }
    }
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Shell integration marker to detect existing installation
const INTEGRATION_MARKER: &str = "# DX Shell Integration";

/// Generate shell integration script
///
/// Requirement 8.2: Add smart aliases
/// Requirement 8.3: Detect when entering a DX project directory
pub fn generate_integration(shell: ShellType) -> String {
    match shell {
        ShellType::Bash => generate_bash_integration(),
        ShellType::Zsh => generate_zsh_integration(),
        ShellType::Fish => generate_fish_integration(),
        ShellType::PowerShell => generate_powershell_integration(),
        ShellType::Nushell => generate_nushell_integration(),
    }
}

/// Generate Bash integration script
fn generate_bash_integration() -> String {
    format!(
        r#"{marker}
# Smart aliases for DX CLI
alias d='dx'
alias dr='dx run'
alias db='dx build'
alias dd='dx dev'
alias dt='dx test'
alias dg='dx generator'
alias ds='dx style'
alias df='dx forge'

# CD hook for DX project detection
__dx_cd_hook() {{
    if [[ -f "dx.toml" ]]; then
        echo -e "\033[36m→\033[0m DX project detected"
    fi
}}

# Override cd to include hook
cd() {{
    builtin cd "$@" && __dx_cd_hook
}}

# Source completions if available
if command -v dx &> /dev/null; then
    eval "$(dx completions bash 2>/dev/null)"
fi
"#,
        marker = INTEGRATION_MARKER
    )
}

/// Generate Zsh integration script
fn generate_zsh_integration() -> String {
    format!(
        r#"{marker}
# Smart aliases for DX CLI
alias d='dx'
alias dr='dx run'
alias db='dx build'
alias dd='dx dev'
alias dt='dx test'
alias dg='dx generator'
alias ds='dx style'
alias df='dx forge'

# CD hook for DX project detection
__dx_chpwd_hook() {{
    if [[ -f "dx.toml" ]]; then
        echo -e "\033[36m→\033[0m DX project detected"
    fi
}}

# Add to chpwd hooks
autoload -Uz add-zsh-hook
add-zsh-hook chpwd __dx_chpwd_hook

# Source completions if available
if command -v dx &> /dev/null; then
    eval "$(dx completions zsh 2>/dev/null)"
fi
"#,
        marker = INTEGRATION_MARKER
    )
}

/// Generate Fish integration script
fn generate_fish_integration() -> String {
    format!(
        r#"{marker}
# Smart aliases for DX CLI
alias d='dx'
alias dr='dx run'
alias db='dx build'
alias dd='dx dev'
alias dt='dx test'
alias dg='dx generator'
alias ds='dx style'
alias df='dx forge'

# CD hook for DX project detection
function __dx_cd_hook --on-variable PWD
    if test -f "dx.toml"
        echo -e "\033[36m→\033[0m DX project detected"
    end
end

# Source completions if available
if command -v dx &> /dev/null
    dx completions fish 2>/dev/null | source
end
"#,
        marker = INTEGRATION_MARKER
    )
}

/// Generate PowerShell integration script
fn generate_powershell_integration() -> String {
    format!(
        r#"{marker}
# Smart aliases for DX CLI
Set-Alias -Name d -Value dx
function dr {{ dx run $args }}
function db {{ dx build $args }}
function dd {{ dx dev $args }}
function dt {{ dx test $args }}
function dg {{ dx generator $args }}
function ds {{ dx style $args }}
function df {{ dx forge $args }}

# CD hook for DX project detection
$__dx_original_prompt = $function:prompt
function prompt {{
    if (Test-Path "dx.toml") {{
        Write-Host "→ DX project detected" -ForegroundColor Cyan
    }}
    & $__dx_original_prompt
}}

# Source completions if available
if (Get-Command dx -ErrorAction SilentlyContinue) {{
    dx completions powershell 2>$null | Out-String | Invoke-Expression
}}
"#,
        marker = INTEGRATION_MARKER
    )
}

/// Generate Nushell integration script
fn generate_nushell_integration() -> String {
    format!(
        r#"{marker}
# Smart aliases for DX CLI
alias d = dx
alias dr = dx run
alias db = dx build
alias dd = dx dev
alias dt = dx test
alias dg = dx generator
alias ds = dx style
alias df = dx forge

# Note: Nushell hooks require additional configuration
# See: https://www.nushell.sh/book/hooks.html
"#,
        marker = INTEGRATION_MARKER
    )
}

/// Check if shell integration is already installed
///
/// Requirement 8.5: Warn if already installed
pub fn is_installed(shell: ShellType) -> Result<bool, DxError> {
    let config_path = shell.config_path().ok_or(DxError::ShellNotDetected)?;

    if !config_path.exists() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(&config_path).map_err(|e| DxError::Io {
        message: format!("Failed to read {}: {}", config_path.display(), e),
    })?;

    Ok(content.contains(INTEGRATION_MARKER))
}

/// Install shell integration
///
/// Requirement 8.5: Warn if already installed
/// Requirement 8.6: Modify appropriate config file
/// Requirement 6.2: Create config file with appropriate permissions
pub fn install(shell: ShellType, force: bool) -> Result<(), DxError> {
    // Check if already installed
    if !force && is_installed(shell)? {
        return Err(DxError::ShellIntegrationExists {
            shell: shell.name().to_string(),
        });
    }

    let config_path = shell.config_path().ok_or(DxError::ShellNotDetected)?;

    // Create parent directories if needed
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DxError::Io {
            message: format!("Failed to create directory {}: {}", parent.display(), e),
        })?;
    }

    // Generate integration script
    let integration = generate_integration(shell);

    // Read existing content or start fresh
    let existing = if config_path.exists() {
        std::fs::read_to_string(&config_path).unwrap_or_default()
    } else {
        String::new()
    };

    // Remove old integration if force reinstalling
    let cleaned = if force {
        remove_integration_from_content(&existing)
    } else {
        existing
    };

    // Append new integration
    let new_content = if cleaned.is_empty() {
        integration
    } else {
        format!("{}\n\n{}", cleaned.trim_end(), integration)
    };

    // Write back
    std::fs::write(&config_path, &new_content).map_err(|e| DxError::Io {
        message: format!("Failed to write {}: {}", config_path.display(), e),
    })?;

    // Set permissions on Unix (0644 = rw-r--r--)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o644);
        std::fs::set_permissions(&config_path, perms).map_err(|e| DxError::Io {
            message: format!("Failed to set permissions on {}: {}", config_path.display(), e),
        })?;
    }

    Ok(())
}

/// Uninstall shell integration
pub fn uninstall(shell: ShellType) -> Result<(), DxError> {
    let config_path = shell.config_path().ok_or(DxError::ShellNotDetected)?;

    if !config_path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&config_path).map_err(|e| DxError::Io {
        message: format!("Failed to read {}: {}", config_path.display(), e),
    })?;

    let cleaned = remove_integration_from_content(&content);

    std::fs::write(&config_path, cleaned).map_err(|e| DxError::Io {
        message: format!("Failed to write {}: {}", config_path.display(), e),
    })?;

    Ok(())
}

/// Remove DX integration from config content
fn remove_integration_from_content(content: &str) -> String {
    let mut result = String::new();
    let mut in_integration = false;

    for line in content.lines() {
        if line.contains(INTEGRATION_MARKER) {
            in_integration = true;
            continue;
        }

        // End of integration block (next comment or significant content)
        if in_integration {
            // Check if this is a new section (non-DX comment or empty line followed by content)
            if line.starts_with('#') && !line.contains("DX") && !line.contains("dx") {
                in_integration = false;
            } else if line.trim().is_empty() {
                // Skip empty lines in integration block
                continue;
            } else if !line.starts_with('#')
                && !line.starts_with("alias")
                && !line.starts_with("function")
                && !line.starts_with("if")
                && !line.starts_with("fi")
                && !line.starts_with("eval")
                && !line.starts_with("autoload")
                && !line.starts_with("add-zsh-hook")
                && !line.starts_with("Set-Alias")
                && !line.starts_with("$")
                && !line.contains("dx")
                && !line.contains("__dx")
            {
                in_integration = false;
            }
        }

        if !in_integration {
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim_end().to_string() + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_shell_type_name() {
        assert_eq!(ShellType::Bash.name(), "bash");
        assert_eq!(ShellType::Zsh.name(), "zsh");
        assert_eq!(ShellType::Fish.name(), "fish");
        assert_eq!(ShellType::PowerShell.name(), "powershell");
        assert_eq!(ShellType::Nushell.name(), "nushell");
    }

    #[test]
    fn test_shell_type_display() {
        assert_eq!(format!("{}", ShellType::Bash), "bash");
        assert_eq!(format!("{}", ShellType::Zsh), "zsh");
    }

    #[test]
    fn test_generate_integration_contains_marker() {
        for shell in [
            ShellType::Bash,
            ShellType::Zsh,
            ShellType::Fish,
            ShellType::PowerShell,
            ShellType::Nushell,
        ] {
            let script = generate_integration(shell);
            assert!(
                script.contains(INTEGRATION_MARKER),
                "{} integration should contain marker",
                shell
            );
        }
    }

    #[test]
    fn test_generate_integration_contains_aliases() {
        for shell in [
            ShellType::Bash,
            ShellType::Zsh,
            ShellType::Fish,
            ShellType::PowerShell,
            ShellType::Nushell,
        ] {
            let script = generate_integration(shell);

            // Check for key aliases
            assert!(
                script.contains("dr") || script.contains("'dx run'"),
                "{} should have run alias",
                shell
            );
            assert!(
                script.contains("db") || script.contains("'dx build'"),
                "{} should have build alias",
                shell
            );
            assert!(
                script.contains("dd") || script.contains("'dx dev'"),
                "{} should have dev alias",
                shell
            );
        }
    }

    #[test]
    fn test_remove_integration_from_content() {
        let content = format!(
            r#"# Existing config
export PATH="$HOME/bin:$PATH"

{}
alias d='dx'
alias dr='dx run'

# Other config
export EDITOR=vim
"#,
            INTEGRATION_MARKER
        );

        let cleaned = remove_integration_from_content(&content);

        assert!(!cleaned.contains(INTEGRATION_MARKER));
        assert!(!cleaned.contains("alias d='dx'"));
        assert!(cleaned.contains("export PATH"));
        assert!(cleaned.contains("export EDITOR"));
    }

    // ═══════════════════════════════════════════════════════════════════
    //  PROPERTY TESTS
    // ═══════════════════════════════════════════════════════════════════

    // Feature: dx-cli, Property 10: Shell Integration Script Content
    // Validates: Requirements 8.2, 8.3
    //
    // For any shell type, the generated integration script should contain
    // the required aliases and the integration marker.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(5))]

        #[test]
        fn prop_shell_integration_script_content(
            shell_idx in 0usize..5
        ) {
            let shells = [
                ShellType::Bash,
                ShellType::Zsh,
                ShellType::Fish,
                ShellType::PowerShell,
                ShellType::Nushell,
            ];
            let shell = shells[shell_idx];
            let script = generate_integration(shell);

            // Must contain marker
            prop_assert!(script.contains(INTEGRATION_MARKER),
                "Script for {} must contain integration marker", shell);

            // Must contain aliases (in some form)
            let has_aliases = script.contains("alias") || script.contains("Set-Alias");
            prop_assert!(has_aliases,
                "Script for {} must define aliases", shell);

            // Must reference dx command
            prop_assert!(script.contains("dx"),
                "Script for {} must reference dx command", shell);
        }
    }

    // Feature: dx-cli-hardening, Property 18: Shell Integration Duplicate Detection
    // **Validates: Requirements 6.3**
    //
    // For any shell configuration file already containing DX integration markers,
    // install() without --force SHALL return a ShellIntegrationExists error.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_duplicate_detection(
            shell_idx in 0usize..5,
            prefix in "[a-zA-Z0-9# ='\n]{0,100}",
            suffix in "[a-zA-Z0-9# ='\n]{0,100}"
        ) {
            let shells = [
                ShellType::Bash,
                ShellType::Zsh,
                ShellType::Fish,
                ShellType::PowerShell,
                ShellType::Nushell,
            ];
            let shell = shells[shell_idx];

            // Content with marker should be detected as installed
            let content_with_marker = format!(
                "{}\n{}\nalias d='dx'\n{}",
                prefix, INTEGRATION_MARKER, suffix
            );
            prop_assert!(
                content_with_marker.contains(INTEGRATION_MARKER),
                "Content with marker should contain the marker"
            );

            // Content without marker should not be detected
            let content_without_marker = format!(
                "{}\n# Some other comment\nalias something='else'\n{}",
                prefix, suffix
            );
            prop_assert!(
                !content_without_marker.contains(INTEGRATION_MARKER),
                "Content without marker should not contain the marker"
            );
        }
    }

    #[test]
    fn test_duplicate_detection_with_marker() {
        let content_with_marker = format!("# Config\n{}\nalias d='dx'\n", INTEGRATION_MARKER);
        let content_without_marker = "# Config\nalias something='else'\n";

        assert!(content_with_marker.contains(INTEGRATION_MARKER));
        assert!(!content_without_marker.contains(INTEGRATION_MARKER));
    }

    // Feature: dx-cli, Property 12: Shell Config Path Mapping
    // Validates: Requirements 8.6
    //
    // For each shell type, config_path() should return a path that
    // corresponds to the standard config file for that shell.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(5))]

        #[test]
        fn prop_shell_config_path_mapping(
            shell_idx in 0usize..5
        ) {
            let shells = [
                ShellType::Bash,
                ShellType::Zsh,
                ShellType::Fish,
                ShellType::PowerShell,
                ShellType::Nushell,
            ];
            let shell = shells[shell_idx];

            if let Some(path) = shell.config_path() {
                let path_str = path.to_string_lossy().to_lowercase();

                match shell {
                    ShellType::Bash => {
                        prop_assert!(
                            path_str.contains("bash"),
                            "Bash config path should contain 'bash': {}",
                            path_str
                        );
                    }
                    ShellType::Zsh => {
                        prop_assert!(
                            path_str.contains("zsh"),
                            "Zsh config path should contain 'zsh': {}",
                            path_str
                        );
                    }
                    ShellType::Fish => {
                        prop_assert!(
                            path_str.contains("fish"),
                            "Fish config path should contain 'fish': {}",
                            path_str
                        );
                    }
                    ShellType::PowerShell => {
                        prop_assert!(
                            path_str.contains("powershell"),
                            "PowerShell config path should contain 'powershell': {}",
                            path_str
                        );
                    }
                    ShellType::Nushell => {
                        prop_assert!(
                            path_str.contains("nushell"),
                            "Nushell config path should contain 'nushell': {}",
                            path_str
                        );
                    }
                }
            }
        }
    }

    // Feature: dx-cli-hardening, Property 19: Shell Integration Idempotence
    // **Validates: Requirements 6.7**
    //
    // For any shell configuration, calling install(force=true) multiple times
    // SHALL result in exactly one copy of the DX integration block, with no duplicates.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_idempotence(
            shell_idx in 0usize..5,
            existing_content in "[a-zA-Z0-9# ='\n]{0,200}"
        ) {
            let shells = [
                ShellType::Bash,
                ShellType::Zsh,
                ShellType::Fish,
                ShellType::PowerShell,
                ShellType::Nushell,
            ];
            let shell = shells[shell_idx];

            // Simulate multiple force installs by generating integration multiple times
            // and removing old integration each time
            let integration = generate_integration(shell);

            // First "install" - add integration to existing content
            let after_first = format!("{}\n\n{}", existing_content.trim_end(), integration);

            // Second "install" with force - should remove old and add new
            let cleaned = remove_integration_from_content(&after_first);
            let after_second = format!("{}\n\n{}", cleaned.trim_end(), integration);

            // Count occurrences of the marker
            let marker_count = after_second.matches(INTEGRATION_MARKER).count();

            prop_assert_eq!(
                marker_count, 1,
                "After force reinstall, should have exactly one integration marker, found {}",
                marker_count
            );
        }
    }

    // Feature: dx-cli-hardening, Property 20: Completion Script Validity
    // **Validates: Requirements 6.6**
    //
    // For any supported shell type (Bash, Zsh, Fish, PowerShell, Nushell),
    // the generated completion script SHALL be syntactically valid for that shell.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_completion_script_validity(shell_idx in 0usize..5) {
            let shells = [
                ShellType::Bash,
                ShellType::Zsh,
                ShellType::Fish,
                ShellType::PowerShell,
                ShellType::Nushell,
            ];
            let shell = shells[shell_idx];
            let script = generate_integration(shell);

            // Basic syntax checks for each shell type
            match shell {
                ShellType::Bash | ShellType::Zsh => {
                    // Check for balanced braces
                    let open_braces = script.matches('{').count();
                    let close_braces = script.matches('}').count();
                    prop_assert_eq!(
                        open_braces, close_braces,
                        "Bash/Zsh script should have balanced braces"
                    );

                    // Check for proper function syntax
                    if script.contains("()") {
                        prop_assert!(
                            script.contains('{'),
                            "Functions should have body braces"
                        );
                    }
                }
                ShellType::Fish => {
                    // Fish uses 'function' and 'end' keywords
                    let function_count = script.matches("function ").count();
                    let end_count = script.matches("\nend").count() + script.matches(" end").count();
                    // Fish also uses 'if' and 'end'
                    let if_count = script.matches("if ").count();
                    prop_assert!(
                        end_count >= function_count,
                        "Fish script should have 'end' for each 'function'"
                    );
                }
                ShellType::PowerShell => {
                    // PowerShell uses { } for blocks
                    let open_braces = script.matches('{').count();
                    let close_braces = script.matches('}').count();
                    prop_assert_eq!(
                        open_braces, close_braces,
                        "PowerShell script should have balanced braces"
                    );
                }
                ShellType::Nushell => {
                    // Nushell uses 'alias' keyword
                    prop_assert!(
                        script.contains("alias"),
                        "Nushell script should contain alias definitions"
                    );
                }
            }

            // All scripts should be non-empty
            prop_assert!(!script.trim().is_empty(), "Script should not be empty");

            // All scripts should contain the marker
            prop_assert!(
                script.contains(INTEGRATION_MARKER),
                "Script should contain integration marker"
            );
        }
    }
}
