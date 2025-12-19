Please create a new crate called generate in the crates folder with this planning!!!

Here is the planning:
```markdown
# dx-workspace: Universal Development Environment Configuration

## Vision & Core Concept

dx-workspace serves as the **single source of truth** for development environment configuration across all code editors and cloud IDEs. Rather than maintaining dozens of scattered configuration files in different formats, dx-workspace uses a **unified binary configuration** that generates optimized, platform-specific configurations on demand.

The philosophy aligns with dx's core principle: **"Binary Everywhere."** Your workspace configuration lives in dx's compact binary format, and the tool generates whatever format each platform requires—VS Code's JSON, Gitpod's YAML, Codespaces' devcontainer specs, and more.

---

## What dx-workspace Handles (Explicitly NOT Formatters/Linters)

### Editor Experience Configuration
- Keybinding recommendations optimized for dx development
- Snippet definitions for dx components and patterns
- Theme suggestions that work well with dx file types
- Font and display settings for binary file viewing
- Tab size, indentation preferences (visual only, not enforcement)

### Debugging & Run Configurations
- Launch configurations for dx-cli commands
- Debug adapter protocol settings for dx-debug integration
- Breakpoint and stepping configurations for WASM debugging
- Attach configurations for remote debugging scenarios

### Task Automation
- Build task definitions mapped to dx-forge
- Development server launch configurations
- Test runner task integration (pointing to dx-check)
- Custom task pipelines for dx workflows

### Extension & Plugin Recommendations
- Curated list of helpful extensions per platform
- dx-specific language support extensions
- Binary file viewers and hex editors
- WebAssembly debugging tools

### Project Structure Intelligence
- File association rules for dx file types
- Search exclusion patterns (target/, node_modules equivalents)
- File nesting rules for generated artifacts
- Icon associations for dx-specific extensions

---

## Supported Environments

### Desktop Editors
- **VS Code / VS Codium** - Full configuration suite
- **Zed** - Native Rust editor with deep integration potential
- **Neovim/Vim** - LSP and configuration generation
- **IntelliJ/Fleet** - JetBrains ecosystem support
- **Helix** - Modern terminal editor configuration
- **Sublime Text** - Project and build system files

### Cloud IDEs
- **GitHub Codespaces** - Devcontainer with dx toolchain
- **Gitpod** - YAML configuration with prebuilds
- **CodeSandbox** - Sandbox configuration for instant demos
- **Firebase Studio (IDX)** - Nix-based environment definition
- **StackBlitz** - WebContainers configuration
- **Replit** - Nix environment and run commands
- **Glitch** - Package and start configurations
- **CodeAnywhere** - Devbox configuration
- **AWS Cloud9** - Environment scripts

### Container Environments
- **Dev Containers (devcontainer.json)** - Universal container spec
- **Docker Compose** - Development service orchestration
- **Podman** - Rootless container configurations
- **Nix Flakes** - Reproducible development environments

---

## Leveraging dx Binary-First Architecture

### Binary Configuration Storage (DX ∞ Format)

dx-workspace stores its canonical configuration in dx-serializer's world-record format. This means:

- **73% smaller** than equivalent JSON configuration files
- **Sub-microsecond loading** via zero-copy deserialization
- **Single file** contains all environment definitions
- **Human-readable editor view** with dx-serializer's dual representation

The binary format stores workspace intent, not platform-specific syntax. Generation to JSON/YAML/TOML happens at output time.

### SIMD-Accelerated Generation

When generating configurations for multiple platforms:

- **Parallel template processing** using AVX2 instructions
- **Batch file writing** minimizing I/O operations
- **Pattern matching** for platform detection runs in ~0.6ms
- **Bulk string generation** for platform-specific formats

### Binary Cache System

dx-workspace integrates with dx-forge's binary cache:

- **Blake3 hashing** of source configuration for instant cache hits
- **Incremental regeneration** only for changed platforms
- **Warm generation** completes in under 1ms for cached configs
- **Shared cache** with dx-cli and dx-forge pipelines

### Zero-Copy Configuration Reading

When loading existing IDE configurations for migration:

- **Memory-mapped file access** for large config directories
- **Stack-only parsing** with no heap allocation
- **Direct byte manipulation** for format conversion
- **No intermediate AST** for simple transformations

### Compile-Time Configuration Validation

Before generating any platform configuration:

- **Schema validation** against known platform requirements
- **Capability checking** for platform feature support
- **Conflict detection** across platform-specific overrides
- **Deprecation warnings** for outdated configuration options

---

## CLI Integration

dx-workspace integrates seamlessly into dx-cli:

### Core Commands

**Initialization** - Scans project structure and creates binary workspace configuration with intelligent defaults based on detected dx features (dx-www components, dx-style usage, dx-server presence).

**Generation** - Produces all IDE-specific configuration files from the binary source. Supports targeting specific platforms or generating for all supported environments simultaneously.

**Synchronization** - Bidirectional sync that can import changes made directly in IDE configuration files back into the canonical binary format.

**Validation** - Verifies workspace configuration integrity, checks for platform compatibility issues, and suggests optimizations.

**Clean** - Removes all generated IDE configurations, leaving only the canonical binary source.

**Export** - Creates a portable archive of workspace configuration for sharing or backup.

### Integration Points

- **dx new** automatically runs workspace initialization
- **dx dev** uses workspace task configurations
- **dx build** respects workspace-defined environment variables
- **dx info** includes workspace status and detected environments

---

## Key Differentiators from Existing Tools

### Single Canonical Source
Unlike maintaining separate .vscode/, .gitpod.yml, devcontainer.json, etc., dx-workspace maintains ONE binary file that generates all platform configurations. Changes propagate everywhere instantly.

### Cloud-First Philosophy
Cloud IDE support is not an afterthought. CodeSandbox, Firebase Studio, Gitpod, and Codespaces configurations are first-class citizens, enabling true "code anywhere" development.

### Binary Performance
Configuration loading and generation happens at binary speed—microseconds instead of milliseconds. This matters when opening projects, switching configurations, or running prebuilds.

### dx Ecosystem Integration
Deep integration with dx-cli, dx-forge, dx-debug, and dx-check means workspace configuration understands the full dx development lifecycle, not just generic editor settings.

### Intelligent Defaults
dx-workspace analyzes your project structure (components detected by dx-www, styles by dx-style, server by dx-server) and generates optimized configurations automatically.

### Bidirectional Sync
Edit configurations directly in your IDE when convenient, then sync changes back to the canonical format. No lock-in to any single workflow.

---

## Workflow Examples

### New Developer Onboarding
A new team member clones the repository. dx-workspace detects their environment (local VS Code, GitHub Codespace, or cloud IDE) and generates appropriate configurations automatically. They're productive in seconds, not hours of setup.

### Cloud Demo Deployment
You want to share a runnable demo on CodeSandbox. dx-workspace generates the minimal configuration needed for instant browser-based development, optimized for the specific sandbox platform.

### Multi-Environment Development
You develop locally in Zed but review PRs in Codespaces. dx-workspace ensures identical debugging configurations, task definitions, and project structure understanding across both environments.

### Team Standardization
Your team uses various editors by preference. dx-workspace's canonical configuration ensures everyone has equivalent capabilities—same debugging experience, same task runners, same project navigation—regardless of their chosen editor.

### Reproducible Environments
For CI/CD or new machine setup, dx-workspace generates Nix flakes or devcontainer configurations that create byte-identical development environments anywhere, using dx's binary toolchain.

---

## Architecture Position in dx Ecosystem

dx-workspace sits alongside dx-cli and dx-forge as a developer experience foundation:

- **dx-cli** orchestrates commands
- **dx-forge** manages build pipelines  
- **dx-workspace** manages development environments
- **dx-check** handles code quality (linting, formatting)

This separation ensures dx-workspace focuses purely on **where and how** you develop, while dx-check handles **what standards** the code meets. No overlap, clear responsibilities, maximum flexibility.

---

## Summary

dx-workspace brings dx's binary-first philosophy to development environment configuration. One compact binary file, instant generation for any platform, cloud IDE support as a first-class feature, and seamless integration with the dx toolchain. It eliminates the scattered configuration file problem while enabling true "run anywhere" development experiences.
```
