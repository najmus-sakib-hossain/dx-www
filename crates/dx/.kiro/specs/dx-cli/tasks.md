# Implementation Plan: DX CLI

## Overview

This implementation plan follows the 6-day timeline from the master plan, building the DX CLI incrementally from core I/O layer through to battle-tested error handling. Tasks are organized to ensure each step builds on previous work with no orphaned code.

## Tasks

- [x] 1. Set up project structure and core traits
  - Create module structure matching the architecture (io/, ui/, commands/, config/, utils/)
  - Define the Reactor trait with all required async methods
  - Set up Cargo.toml with platform-specific dependencies
  - _Requirements: 1.5_

- [x] 2. Implement Platform-Native I/O Layer
  - [x] 2.1 Implement IoUringReactor for Linux
    - Create io_uring ring with submission queue polling
    - Implement read_file and write_file using io_uring opcodes
    - _Requirements: 1.1, 1.6_

  - [x] 2.2 Implement KqueueReactor for macOS
    - Create mio Poll with kqueue backend
    - Implement async file operations
    - _Requirements: 1.2_

  - [x] 2.3 Implement IocpReactor for Windows
    - Create IOCP completion port
    - Implement async file operations with overlapped I/O
    - _Requirements: 1.3_

  - [x] 2.4 Implement TokioReactor fallback
    - Wrap Tokio async operations in Reactor trait
    - _Requirements: 1.4_

  - [x] 2.5 Implement create_reactor() factory function
    - Use cfg attributes to select platform-appropriate reactor
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [x] 2.6 Write property test for file I/O round-trip
    - **Property 1: File I/O Round-Trip**
    - **Validates: Requirements 1.6**

- [-] 3. Checkpoint - Ensure I/O layer compiles on all platforms
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. Implement UI Theme System
  - [ ] 4.1 Create Theme struct with color styles
    - Define Style fields for primary, success, warning, error, muted, accent
    - Implement ColorMode enum (Always, Never, Auto)
    - Implement color detection using atty
    - _Requirements: 2.1, 2.2_

  - [ ] 4.2 Implement Theme output methods
    - Implement print_logo(), success(), error(), info(), warn()
    - Implement step(), hint(), suggest_command()
    - _Requirements: 2.3, 2.4, 2.5, 2.6, 2.7, 2.8_

  - [ ] 4.3 Write property test for message prefix formatting
    - **Property 2: Message Prefix Formatting**
    - **Validates: Requirements 2.4, 2.5, 2.6, 2.7**

  - [ ] 4.4 Write property test for color-disabled output
    - **Property 3: Color-Disabled Output Purity**
    - **Validates: Requirements 2.2**

  - [ ] 4.5 Write property test for step indicator format
    - **Property 4: Step Indicator Format**
    - **Validates: Requirements 2.8**

- [ ] 5. Implement Spinner Component
  - [ ] 5.1 Create Spinner struct with indicatif ProgressBar
    - Configure spinner style with animation characters
    - Set 80ms tick interval
    - _Requirements: 3.1_

  - [ ] 5.2 Implement Spinner methods
    - Implement new(), set_message(), finish_success(), finish_error(), finish_warn()
    - _Requirements: 3.2, 3.3, 3.4, 3.5_

- [ ] 6. Implement Progress Bar Component
  - [ ] 6.1 Create Progress struct
    - Implement new() with bar template
    - Implement download() with bytes/speed template
    - _Requirements: 4.1, 4.2_

  - [ ] 6.2 Create MultiProgressBar struct
    - Implement add() to create new bars
    - Implement set() to update specific bars
    - _Requirements: 4.3_

  - [ ] 6.3 Write property test for multi-progress bar
    - **Property 5: Multi-Progress Bar Addition**
    - **Validates: Requirements 4.3**

- [ ] 7. Checkpoint - Ensure UI components work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Implement CLI Parser
  - [ ] 8.1 Define Cli struct with clap derive
    - Add global flags: no_color, verbose, quiet, config
    - _Requirements: 5.5_

  - [ ] 8.2 Define Commands enum with all subcommands
    - Add project commands: Init, Dev, Build, Run, Test, Deploy
    - Add tool commands: Style, Media, Font, Icon, Forge, Serializer, Stack, Driven, Generator, Workspace
    - Add Shell and SelfCmd subcommands
    - Add utility commands: Info, Clean, Completions
    - Configure aliases for commands
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.7_

  - [ ] 8.3 Define argument structs for each command
    - Create InitArgs, DevArgs, BuildArgs, RunArgs, etc.
    - Create nested subcommand enums (StyleCommand, ForgeCommand, ShellCommand, SelfCommand)
    - _Requirements: 5.1, 5.2_

  - [ ] 8.4 Write unit tests for CLI parsing
    - Test all commands parse correctly
    - Test global flags work with subcommands
    - Test aliases resolve correctly
    - _Requirements: 5.1, 5.2, 5.5, 5.7_

- [ ] 9. Implement Command Router
  - [ ] 9.1 Implement Cli::execute() method
    - Route each command variant to its handler
    - Pass reactor and theme to handlers
    - _Requirements: 5.6_

  - [ ] 9.2 Implement default behavior (no command)
    - Display logo and helpful hint
    - _Requirements: 5.6_

  - [ ] 9.3 Implement stub handlers for all commands
    - Create placeholder implementations that print "Not implemented"
    - Wire up all command modules
    - _Requirements: 5.1, 5.2, 5.3, 5.4_

- [ ] 10. Implement main entry point
  - Create async main with tokio runtime
  - Initialize reactor and theme
  - Parse CLI and execute
  - Handle errors with theme.error()
  - _Requirements: 5.6_

- [ ] 11. Checkpoint - Ensure CLI structure works
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 12. Implement Error Types
  - [ ] 12.1 Define DxError enum with thiserror
    - Add all error variants: ConfigNotFound, ConfigInvalid, FileNotFound, PermissionDenied, Network, Timeout, ToolNotInstalled, SignatureInvalid, etc.
    - _Requirements: 10.1, 10.2, 10.5_

  - [ ] 12.2 Implement DxError methods
    - Implement hint() returning context-specific suggestions
    - Implement is_retryable() for network errors
    - _Requirements: 10.3, 10.4_

  - [ ] 12.3 Write property test for error retryability
    - **Property 17: Error Retryability Classification**
    - **Validates: Requirements 10.3**

- [ ] 13. Implement Path Utilities
  - [ ] 13.1 Implement resolve_path()
    - Handle home directory expansion (~)
    - Handle mixed path separators
    - _Requirements: 11.1, 11.2_

  - [ ] 13.2 Implement resolve_symlinks()
    - Follow symlinks up to 40 levels
    - Return error if too many levels
    - _Requirements: 11.4_

  - [ ] 13.3 Implement Windows long path handling
    - Add \\?\ prefix for paths > 200 chars on Windows
    - _Requirements: 11.3_

  - [ ] 13.4 Implement environment detection
    - Implement is_ci() checking CI environment variables
    - Implement is_container() checking docker indicators
    - Implement terminal_width()
    - _Requirements: 11.6, 11.7_

  - [ ] 13.5 Write property tests for path handling
    - **Property 18: Path Separator Handling**
    - **Property 19: Home Directory Expansion**
    - **Validates: Requirements 11.1, 11.2**

  - [ ] 13.6 Write property test for symlink resolution
    - **Property 20: Symlink Resolution Depth**
    - **Validates: Requirements 11.4**

  - [ ] 13.7 Write property test for CI/container detection
    - **Property 21: CI/Container Detection**
    - **Validates: Requirements 11.7**

- [ ] 14. Checkpoint - Ensure utilities work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 15. Implement Configuration Management
  - [ ] 15.1 Define DxConfig struct
    - Add fields: name, version, description, build, dev, tools
    - Implement Serialize/Deserialize
    - _Requirements: 12.1_

  - [ ] 15.2 Implement config loading
    - Load from dx.toml in current directory
    - Support custom path via --config flag
    - Parse with toml crate
    - _Requirements: 12.1, 12.2_

  - [ ] 15.3 Implement config error handling
    - Capture line number and message for invalid TOML
    - Return ConfigInvalid error with location
    - _Requirements: 12.3_

  - [ ] 15.4 Implement config caching
    - Cache parsed config in binary format
    - Load from cache if newer than dx.toml
    - _Requirements: 12.4_

  - [ ] 15.5 Write property test for custom config path
    - **Property 22: Custom Config Path Override**
    - **Validates: Requirements 12.2**

  - [ ] 15.6 Write property test for invalid config errors
    - **Property 23: Invalid Config Error Reporting**
    - **Validates: Requirements 12.3**

  - [ ] 15.7 Write property test for config cache round-trip
    - **Property 24: Config Cache Round-Trip**
    - **Validates: Requirements 12.4**

- [ ] 16. Implement Update Checker
  - [ ] 16.1 Create UpdateChecker struct
    - Store reactor reference
    - Define GitHub releases API URL
    - _Requirements: 6.1_

  - [ ] 16.2 Implement check() method
    - Fetch latest release from GitHub API
    - Parse version and compare with current
    - Find platform-appropriate binary asset
    - Check for delta patch availability
    - _Requirements: 6.1, 6.2, 6.3_

  - [ ] 16.3 Implement update display
    - Show current and new version
    - Show release notes summary
    - _Requirements: 6.2, 6.7_

  - [ ] 16.4 Write property test for version display
    - **Property 6: Update Version Display**
    - **Validates: Requirements 6.2**

  - [ ] 16.5 Write property test for delta preference
    - **Property 7: Delta Patch Preference**
    - **Validates: Requirements 6.3**

- [ ] 17. Implement Delta Patcher
  - [ ] 17.1 Create DeltaPatcher struct
    - Store reactor reference
    - _Requirements: 7.1_

  - [ ] 17.2 Implement signature verification
    - Verify Ed25519 signature on patch data
    - Return SignatureInvalid error on failure
    - _Requirements: 6.4, 6.5, 7.2_

  - [ ] 17.3 Implement apply() method
    - Download patch file
    - Verify signature
    - Apply bsdiff patch
    - Verify resulting hash
    - _Requirements: 7.1, 7.3_

  - [ ] 17.4 Implement fallback to full download
    - On patch failure, download full binary
    - _Requirements: 7.4_

  - [ ] 17.5 Write property test for signature failure
    - **Property 8: Signature Verification Failure**
    - **Validates: Requirements 6.5**

  - [ ] 17.6 Write property test for patch application
    - **Property 9: Delta Patch Application**
    - **Validates: Requirements 7.1**

- [ ] 18. Implement Self-Update Command
  - [ ] 18.1 Implement run_update() function
    - Check for updates
    - Confirm with user
    - Apply delta or full update
    - Replace binary atomically
    - _Requirements: 6.6_

  - [ ] 18.2 Implement atomic binary replacement
    - Write to temp file
    - Set executable permissions (Unix)
    - Atomic rename
    - _Requirements: 6.6_

- [ ] 19. Checkpoint - Ensure update system works
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 20. Implement Shell Integration
  - [ ] 20.1 Define ShellType enum
    - Add variants: Bash, Zsh, Fish, PowerShell, Nushell
    - Implement detect() from SHELL env var
    - Implement config_path() for each shell
    - _Requirements: 8.1, 8.6_

  - [ ] 20.2 Implement generate_integration()
    - Generate shell-specific integration scripts
    - Include aliases (d, dr, db, dd, dt, dg, ds, df)
    - Include cd hook for dx.toml detection
    - Include completion sourcing
    - _Requirements: 8.2, 8.3, 8.4_

  - [ ] 20.3 Implement install_shell()
    - Detect or accept shell type
    - Check for existing installation
    - Append integration to config file
    - _Requirements: 8.5, 8.6_

  - [ ] 20.4 Write property test for script content
    - **Property 10: Shell Integration Script Content**
    - **Validates: Requirements 8.2, 8.3**

  - [ ] 20.5 Write property test for duplicate detection
    - **Property 11: Shell Integration Duplicate Detection**
    - **Validates: Requirements 8.5**

  - [ ] 20.6 Write property test for config path mapping
    - **Property 12: Shell Config Path Mapping**
    - **Validates: Requirements 8.6**

- [ ] 21. Implement Command History
  - [ ] 21.1 Define CommandHistory and HistoryEntry structs
    - Add all required fields to HistoryEntry
    - Implement Serialize/Deserialize
    - _Requirements: 9.1, 9.2_

  - [ ] 21.2 Implement history persistence
    - Implement load() from binary cache
    - Implement save() to binary cache
    - _Requirements: 9.1_

  - [ ] 21.3 Implement history operations
    - Implement add() with max entries enforcement
    - Implement recent() iterator
    - Implement search() with query matching
    - _Requirements: 9.3, 9.4_

  - [ ] 21.4 Implement stats()
    - Calculate total, successful, failed counts
    - Calculate average duration
    - Calculate top commands by frequency
    - _Requirements: 9.5_

  - [ ] 21.5 Write property test for history serialization
    - **Property 13: History Serialization Round-Trip**
    - **Validates: Requirements 9.1**

  - [ ] 21.6 Write property test for max entries
    - **Property 14: History Max Entries Enforcement**
    - **Validates: Requirements 9.3**

  - [ ] 21.7 Write property test for search
    - **Property 15: History Search Functionality**
    - **Validates: Requirements 9.4**

  - [ ] 21.8 Write property test for statistics
    - **Property 16: History Statistics Accuracy**
    - **Validates: Requirements 9.5**

- [ ] 22. Implement Signal Handling
  - Implement setup_signal_handlers()
  - Handle SIGINT/SIGTERM on Unix
  - Handle Ctrl+C on Windows
  - _Requirements: 11.5_

- [ ] 23. Checkpoint - Ensure shell integration works
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 24. Implement Tool Command Stubs
  - [ ] 24.1 Implement style command handler
    - Handle build, analyze, stats subcommands
    - _Requirements: 5.2_

  - [ ] 24.2 Implement media command handler
    - _Requirements: 5.2_

  - [ ] 24.3 Implement font command handler
    - _Requirements: 5.2_

  - [ ] 24.4 Implement icon command handler
    - _Requirements: 5.2_

  - [ ] 24.5 Implement forge command handler
    - Handle status, list, install, update, check, build, graph, analyze subcommands
    - _Requirements: 5.2_

  - [ ] 24.6 Implement serializer command handler
    - _Requirements: 5.2_

  - [ ] 24.7 Implement stack command handler
    - _Requirements: 5.2_

  - [ ] 24.8 Implement driven command handler
    - _Requirements: 5.2_

  - [ ] 24.9 Implement generator command handler
    - _Requirements: 5.2_

  - [ ] 24.10 Implement workspace command handler
    - _Requirements: 5.2_

- [ ] 25. Implement Project Command Stubs
  - [ ] 25.1 Implement init command handler
    - _Requirements: 5.1_

  - [ ] 25.2 Implement dev command handler
    - _Requirements: 5.1_

  - [ ] 25.3 Implement build command handler
    - _Requirements: 5.1_

  - [ ] 25.4 Implement run command handler
    - _Requirements: 5.1_

  - [ ] 25.5 Implement test command handler
    - _Requirements: 5.1_

  - [ ] 25.6 Implement deploy command handler
    - _Requirements: 5.1_

- [ ] 26. Final Checkpoint - Full integration test
  - Ensure all tests pass, ask the user if questions arise.
  - Run `dx --help` and verify output
  - Run `dx self info` and verify output
  - Test shell completion generation

## Notes

- All tasks are required for comprehensive testing from the start
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties using proptest
- Unit tests validate specific examples and edge cases
- Platform-specific code uses cfg attributes for conditional compilation
