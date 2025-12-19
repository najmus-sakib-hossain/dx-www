//! Docker Compose configuration generator.
//!
//! Generates:
//! - docker-compose.yml
//! - Dockerfile.dev

use super::{ContainerGenerator, GeneratedFile};
use crate::{Result, WorkspaceConfig};
use std::fs;
use std::path::Path;

/// Docker Compose configuration generator.
#[derive(Debug, Default)]
pub struct DockerComposeGenerator;

impl DockerComposeGenerator {
    /// Create a new Docker Compose generator.
    pub fn new() -> Self {
        Self
    }

    /// Generate docker-compose.yml content.
    fn generate_compose(&self, config: &WorkspaceConfig) -> String {
        let mut lines = Vec::new();

        lines.push("# dx-workspace generated Docker Compose configuration".to_string());
        lines.push("version: '3.8'".to_string());
        lines.push(String::new());
        lines.push("services:".to_string());

        // Development service
        lines.push("  dev:".to_string());
        lines.push("    build:".to_string());
        lines.push("      context: .".to_string());
        lines.push("      dockerfile: Dockerfile.dev".to_string());
        lines.push("    volumes:".to_string());
        lines.push("      - .:/workspace".to_string());
        lines.push("      - cargo-cache:/root/.cargo".to_string());
        lines.push("      - target-cache:/workspace/target".to_string());
        lines.push("    working_dir: /workspace".to_string());

        if config.detected_features.has_dx_server || config.detected_features.has_dx_www {
            lines.push("    ports:".to_string());
            lines.push("      - \"3000:3000\"".to_string());
            lines.push("      - \"8080:8080\"".to_string());
        }

        lines.push("    environment:".to_string());
        lines.push("      - RUST_BACKTRACE=1".to_string());
        lines.push("      - CARGO_HOME=/root/.cargo".to_string());
        lines.push("    command: cargo watch -x run".to_string());
        lines.push("    tty: true".to_string());
        lines.push("    stdin_open: true".to_string());

        lines.push(String::new());
        lines.push("volumes:".to_string());
        lines.push("  cargo-cache:".to_string());
        lines.push("  target-cache:".to_string());

        lines.join("\n")
    }

    /// Generate Dockerfile.dev content.
    fn generate_dockerfile(&self, config: &WorkspaceConfig) -> String {
        let mut lines = Vec::new();

        lines.push("# dx-workspace generated development Dockerfile".to_string());
        lines.push("FROM rust:latest".to_string());
        lines.push(String::new());

        // Install system dependencies
        lines.push("# Install system dependencies".to_string());
        lines.push("RUN apt-get update && apt-get install -y \\".to_string());
        lines.push("    pkg-config \\".to_string());
        lines.push("    libssl-dev \\".to_string());
        lines.push("    git \\".to_string());
        lines.push("    && rm -rf /var/lib/apt/lists/*".to_string());
        lines.push(String::new());

        // Install Rust tools
        lines.push("# Install Rust development tools".to_string());
        lines.push("RUN rustup component add clippy rustfmt rust-analyzer".to_string());

        if config.detected_features.has_dx_client {
            lines.push(String::new());
            lines.push("# Install WASM target".to_string());
            lines.push("RUN rustup target add wasm32-unknown-unknown".to_string());
            lines.push("RUN cargo install wasm-pack".to_string());
        }

        lines.push(String::new());
        lines.push("# Install cargo-watch for development".to_string());
        lines.push("RUN cargo install cargo-watch".to_string());

        lines.push(String::new());
        lines.push("# Set working directory".to_string());
        lines.push("WORKDIR /workspace".to_string());
        lines.push(String::new());

        lines.push("# Default command".to_string());
        lines.push("CMD [\"cargo\", \"watch\", \"-x\", \"run\"]".to_string());

        lines.join("\n")
    }
}

impl ContainerGenerator for DockerComposeGenerator {
    fn generate(&self, config: &WorkspaceConfig, output_dir: &Path) -> Result<Vec<GeneratedFile>> {
        let mut files = Vec::new();

        // Generate docker-compose.yml
        let compose_content = self.generate_compose(config);
        files.push(GeneratedFile::new("docker-compose.yml", compose_content.clone()));

        let compose_path = output_dir.join("docker-compose.yml");
        fs::write(&compose_path, &compose_content).map_err(|e| crate::Error::io(&compose_path, e))?;

        // Generate Dockerfile.dev
        let dockerfile_content = self.generate_dockerfile(config);
        files.push(GeneratedFile::new("Dockerfile.dev", dockerfile_content.clone()));

        let dockerfile_path = output_dir.join("Dockerfile.dev");
        fs::write(&dockerfile_path, &dockerfile_content)
            .map_err(|e| crate::Error::io(&dockerfile_path, e))?;

        Ok(files)
    }

    fn exists(&self, project_dir: &Path) -> bool {
        project_dir.join("docker-compose.yml").exists()
            || project_dir.join("compose.yaml").exists()
    }

    fn clean(&self, project_dir: &Path) -> Result<()> {
        let compose_path = project_dir.join("docker-compose.yml");
        if compose_path.exists() {
            fs::remove_file(&compose_path).map_err(|e| crate::Error::io(&compose_path, e))?;
        }

        let dockerfile_path = project_dir.join("Dockerfile.dev");
        if dockerfile_path.exists() {
            fs::remove_file(&dockerfile_path).map_err(|e| crate::Error::io(&dockerfile_path, e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_compose() {
        let mut config = WorkspaceConfig::new("test");
        config.detected_features.has_dx_server = true;

        let generator = DockerComposeGenerator::new();
        let content = generator.generate_compose(&config);

        assert!(content.contains("version: '3.8'"));
        assert!(content.contains("3000:3000"));
    }

    #[test]
    fn test_generate_dockerfile() {
        let mut config = WorkspaceConfig::new("test");
        config.detected_features.has_dx_client = true;

        let generator = DockerComposeGenerator::new();
        let content = generator.generate_dockerfile(&config);

        assert!(content.contains("FROM rust:latest"));
        assert!(content.contains("wasm32-unknown-unknown"));
        assert!(content.contains("wasm-pack"));
    }
}
