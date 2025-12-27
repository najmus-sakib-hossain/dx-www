//! Template files for missing documentation
//!
//! Provides templates for README.md, CHANGELOG.md, and LICENSE files
//! following DX ecosystem standards.

/// Template generator for documentation files
pub struct Templates;

impl Templates {
    /// Generate a README.md template for a crate
    ///
    /// # Arguments
    /// * `crate_name` - The name of the crate (e.g., "dx-serializer")
    /// * `description` - A brief description of the crate
    /// * `lib_name` - The library name with underscores (e.g., "dx_serializer")
    pub fn readme(crate_name: &str, description: &str, lib_name: &str) -> String {
        format!(
            r#"# {crate_name}

[![Crates.io](https://img.shields.io/crates/v/{crate_name}.svg)](https://crates.io/crates/{crate_name})
[![Documentation](https://docs.rs/{crate_name}/badge.svg)](https://docs.rs/{crate_name})
[![License](https://img.shields.io/crates/l/{crate_name}.svg)](LICENSE)

{description}

## Overview

{crate_name} is part of the DX ecosystem, providing [describe main functionality here].

## Features

- Feature 1: [Description]
- Feature 2: [Description]
- Feature 3: [Description]

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
{crate_name} = "0.1"
```

## Usage

```rust
use {lib_name}::prelude::*;

fn main() {{
    // Example usage
}}
```

## Documentation

Full documentation is available at [docs.rs/{crate_name}](https://docs.rs/{crate_name}).

## License

This project is licensed under the MIT OR Apache-2.0 license. See [LICENSE](LICENSE) for details.
"#
        )
    }

    /// Generate a README.md template with minimal placeholders
    pub fn readme_minimal(crate_name: &str, description: Option<&str>) -> String {
        let desc = description.unwrap_or("[Brief description of the crate]");
        let lib_name = crate_name.replace('-', "_");
        Self::readme(crate_name, desc, &lib_name)
    }

    /// Generate a CHANGELOG.md template following Keep a Changelog format
    ///
    /// # Arguments
    /// * `crate_name` - The name of the crate
    pub fn changelog(crate_name: &str) -> String {
        format!(
            r#"# Changelog

All notable changes to {crate_name} will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - YYYY-MM-DD

### Added

- Initial implementation
"#
        )
    }

    /// Generate a MIT license file
    pub fn license_mit() -> String {
        let year = chrono_year();
        format!(
            r#"MIT License

Copyright (c) {year} DX Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#
        )
    }

    /// Generate an Apache 2.0 license file
    pub fn license_apache() -> String {
        let year = chrono_year();
        format!(
            r#"                              Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
   other entities that control, are controlled by, or are under common
   control with that entity. For the purposes of this definition,
   "control" means (i) the power, direct or indirect, to cause the
   direction or management of such entity, whether by contract or
   otherwise, or (ii) ownership of fifty percent (50%) or more of the
   outstanding shares, or (iii) beneficial ownership of such entity.

   "You" (or "Your") shall mean an individual or Legal Entity
   exercising permissions granted by this License.

   "Source" form shall mean the preferred form for making modifications,
   including but not limited to software source code, documentation
   source, and configuration files.

   "Object" form shall mean any form resulting from mechanical
   transformation or translation of a Source form, including but
   not limited to compiled object code, generated documentation,
   and conversions to other media types.

   "Work" shall mean the work of authorship, whether in Source or
   Object form, made available under the License, as indicated by a
   copyright notice that is included in or attached to the work.

   "Derivative Works" shall mean any work, whether in Source or Object
   form, that is based on (or derived from) the Work and for which the
   editorial revisions, annotations, elaborations, or other modifications
   represent, as a whole, an original work of authorship.

   "Contribution" shall mean any work of authorship, including
   the original version of the Work and any modifications or additions
   to that Work or Derivative Works thereof, that is intentionally
   submitted to the Licensor for inclusion in the Work by the copyright owner.

   "Contributor" shall mean Licensor and any individual or Legal Entity
   on behalf of whom a Contribution has been received by Licensor and
   subsequently incorporated within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   copyright license to reproduce, prepare Derivative Works of,
   publicly display, publicly perform, sublicense, and distribute the
   Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   (except as stated in this section) patent license to make, have made,
   use, offer to sell, sell, import, and otherwise transfer the Work.

4. Redistribution. You may reproduce and distribute copies of the
   Work or Derivative Works thereof in any medium, with or without
   modifications, and in Source or Object form, provided that You
   meet the following conditions:

   (a) You must give any other recipients of the Work or
       Derivative Works a copy of this License; and

   (b) You must cause any modified files to carry prominent notices
       stating that You changed the files; and

   (c) You must retain, in the Source form of any Derivative Works
       that You distribute, all copyright, patent, trademark, and
       attribution notices from the Source form of the Work; and

   (d) If the Work includes a "NOTICE" text file as part of its
       distribution, then any Derivative Works that You distribute must
       include a readable copy of the attribution notices contained
       within such NOTICE file.

5. Submission of Contributions. Unless You explicitly state otherwise,
   any Contribution intentionally submitted for inclusion in the Work
   by You to the Licensor shall be under the terms and conditions of
   this License, without any additional terms or conditions.

6. Trademarks. This License does not grant permission to use the trade
   names, trademarks, service marks, or product names of the Licensor.

7. Disclaimer of Warranty. Unless required by applicable law or
   agreed to in writing, Licensor provides the Work on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

8. Limitation of Liability. In no event and under no legal theory,
   whether in tort (including negligence), contract, or otherwise,
   shall any Contributor be liable to You for damages.

9. Accepting Warranty or Additional Liability. While redistributing
   the Work or Derivative Works thereof, You may choose to offer,
   and charge a fee for, acceptance of support, warranty, indemnity,
   or other liability obligations and/or rights consistent with this
   License.

END OF TERMS AND CONDITIONS

Copyright {year} DX Contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
"#
        )
    }

    /// Generate a dual MIT/Apache-2.0 license file (combined)
    pub fn license_dual() -> String {
        let year = chrono_year();
        format!(
            r#"This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Copyright (c) {year} DX Contributors
"#
        )
    }

    /// Generate a .gitignore file for crates with build artifacts
    pub fn gitignore() -> &'static str {
        r#"/target/
Cargo.lock
*.swp
*.swo
*~
.DS_Store
"#
    }

    /// Generate a CONTRIBUTING.md file for top-level tools
    pub fn contributing(crate_name: &str) -> String {
        format!(
            r#"# Contributing to {crate_name}

Thank you for your interest in contributing to {crate_name}!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/dx.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature`
8. Submit a pull request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all tests pass (`cargo test`)
- Ensure no clippy warnings (`cargo clippy`)
- Add documentation for public APIs
- Write tests for new functionality

## Reporting Issues

- Use the GitHub issue tracker
- Include steps to reproduce
- Include expected vs actual behavior
- Include Rust version and OS

## License

By contributing, you agree that your contributions will be licensed under the
MIT OR Apache-2.0 license.
"#
        )
    }
}

/// Get the current year for license files
fn chrono_year() -> u32 {
    // Use a simple approach without external dependencies
    // In production, you might want to use the `chrono` crate
    2025
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme_template() {
        let readme = Templates::readme("dx-test", "A test crate", "dx_test");
        assert!(readme.contains("# dx-test"));
        assert!(readme.contains("A test crate"));
        assert!(readme.contains("dx_test"));
        assert!(readme.contains("crates.io"));
        assert!(readme.contains("docs.rs"));
        assert!(readme.contains("MIT OR Apache-2.0"));
    }

    #[test]
    fn test_readme_minimal() {
        let readme = Templates::readme_minimal("dx-test", Some("Test description"));
        assert!(readme.contains("# dx-test"));
        assert!(readme.contains("Test description"));
    }

    #[test]
    fn test_changelog_template() {
        let changelog = Templates::changelog("dx-test");
        assert!(changelog.contains("# Changelog"));
        assert!(changelog.contains("dx-test"));
        assert!(changelog.contains("Keep a Changelog"));
        assert!(changelog.contains("Semantic Versioning"));
        assert!(changelog.contains("[Unreleased]"));
    }

    #[test]
    fn test_license_mit() {
        let license = Templates::license_mit();
        assert!(license.contains("MIT License"));
        assert!(license.contains("DX Contributors"));
        assert!(license.contains("Permission is hereby granted"));
    }

    #[test]
    fn test_license_apache() {
        let license = Templates::license_apache();
        assert!(license.contains("Apache License"));
        assert!(license.contains("Version 2.0"));
        assert!(license.contains("DX Contributors"));
    }

    #[test]
    fn test_license_dual() {
        let license = Templates::license_dual();
        assert!(license.contains("Apache License"));
        assert!(license.contains("MIT license"));
        assert!(license.contains("at your option"));
    }

    #[test]
    fn test_gitignore() {
        let gitignore = Templates::gitignore();
        assert!(gitignore.contains("/target/"));
        assert!(gitignore.contains("Cargo.lock"));
    }

    #[test]
    fn test_contributing() {
        let contributing = Templates::contributing("dx-test");
        assert!(contributing.contains("# Contributing to dx-test"));
        assert!(contributing.contains("cargo test"));
        assert!(contributing.contains("MIT OR Apache-2.0"));
    }
}
