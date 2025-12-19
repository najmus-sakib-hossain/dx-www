# Requirements Document

## Introduction

`dx-js-monorepo` provides comprehensive JavaScript/TypeScript monorepo and workspace support for the DX ecosystem. This crate enables developers to manage multi-package repositories with features comparable to npm/yarn/pnpm workspaces and Bun workspaces, while leveraging DX's binary-first architecture for superior performance.

The crate integrates with `dx-js-package-manager` for dependency resolution, `dx-js-bundler` for builds, and `dx-js-test-runner` for testing across workspace packages.

## Glossary

- **Workspace_Manager**: The core component that discovers, validates, and manages workspace configurations
- **Package_Resolver**: The component responsible for resolving inter-workspace dependencies and hoisting
- **Dependency_Graph**: The directed acyclic graph representing package dependencies within the workspace
- **Link_Manager**: The component that creates and manages symlinks between workspace packages
- **Task_Runner**: The component that executes scripts/commands across workspace packages with proper ordering
- **Change_Detector**: The component that identifies which packages have changed since a reference point
- **Workspace**: A collection of related packages managed together in a single repository
- **Package**: An individual npm-compatible package within a workspace
- **Root_Package**: The top-level package.json that defines workspace configuration
- **Hoisting**: The process of deduplicating dependencies by moving them to a common ancestor directory
- **Topological_Order**: The order in which packages must be processed based on their dependency relationships

## Requirements

### Requirement 1: Workspace Discovery and Configuration

**User Story:** As a developer, I want the system to automatically discover and parse workspace configurations, so that I can manage multiple packages without manual setup.

#### Acceptance Criteria

1. WHEN a workspace root is specified, THE Workspace_Manager SHALL parse `package.json` workspace globs (npm/yarn style)
2. WHEN a workspace root is specified, THE Workspace_Manager SHALL parse `pnpm-workspace.yaml` configurations
3. WHEN a workspace root is specified, THE Workspace_Manager SHALL parse `bun.lockb` workspace metadata
4. WHEN workspace globs are provided, THE Workspace_Manager SHALL expand globs to discover all matching package directories
5. WHEN a discovered directory contains a valid `package.json`, THE Workspace_Manager SHALL register it as a workspace package
6. IF a workspace glob matches no directories, THEN THE Workspace_Manager SHALL emit a warning but continue processing
7. IF a discovered `package.json` is malformed, THEN THE Workspace_Manager SHALL return a descriptive error with file location
8. WHEN multiple workspace configuration formats exist, THE Workspace_Manager SHALL use precedence order: bun > pnpm > npm/yarn

### Requirement 2: Dependency Graph Construction

**User Story:** As a developer, I want the system to build an accurate dependency graph of my workspace, so that operations can be performed in the correct order.

#### Acceptance Criteria

1. WHEN workspace packages are discovered, THE Dependency_Graph SHALL construct a directed graph of inter-package dependencies
2. WHEN building the graph, THE Dependency_Graph SHALL include `dependencies`, `devDependencies`, `peerDependencies`, and `optionalDependencies`
3. WHEN a package references another workspace package by name, THE Dependency_Graph SHALL create an edge between them
4. WHEN a package references a workspace package with `workspace:*` protocol, THE Dependency_Graph SHALL resolve to the local version
5. WHEN a package references a workspace package with `workspace:^` or `workspace:~` protocol, THE Dependency_Graph SHALL resolve with the appropriate semver range
6. IF a circular dependency is detected, THEN THE Dependency_Graph SHALL return an error listing the cycle path
7. WHEN the graph is complete, THE Dependency_Graph SHALL provide topological ordering for build/test operations
8. WHEN queried for dependents, THE Dependency_Graph SHALL return all packages that depend on a given package

### Requirement 3: Workspace Package Linking

**User Story:** As a developer, I want workspace packages to be automatically linked, so that changes in one package are immediately available to dependent packages.

#### Acceptance Criteria

1. WHEN workspace packages are resolved, THE Link_Manager SHALL create symlinks in `node_modules` for inter-workspace dependencies
2. WHEN creating symlinks, THE Link_Manager SHALL use relative paths for portability
3. WHEN a workspace package has a `bin` field, THE Link_Manager SHALL create executable symlinks in `.bin` directories
4. IF symlink creation fails due to permissions, THEN THE Link_Manager SHALL attempt junction points on Windows or return a descriptive error
5. WHEN a package is added to the workspace, THE Link_Manager SHALL update all dependent package links
6. WHEN a package is removed from the workspace, THE Link_Manager SHALL remove stale symlinks from dependent packages
7. WHEN links are created, THE Link_Manager SHALL verify link targets exist and are valid packages

### Requirement 4: Dependency Hoisting

**User Story:** As a developer, I want shared dependencies to be hoisted to reduce disk usage and installation time, so that my workspace is efficient.

#### Acceptance Criteria

1. WHEN installing dependencies, THE Package_Resolver SHALL identify common dependencies across workspace packages
2. WHEN multiple packages require the same dependency version, THE Package_Resolver SHALL hoist it to the workspace root
3. WHEN packages require different versions of the same dependency, THE Package_Resolver SHALL keep conflicting versions in their respective package directories
4. WHEN hoisting is disabled via configuration, THE Package_Resolver SHALL install all dependencies in their respective package directories
5. WHEN a hoisted dependency has peer dependencies, THE Package_Resolver SHALL ensure peer requirements are satisfied at the hoisted location
6. IF hoisting would break a package's resolution, THEN THE Package_Resolver SHALL keep that dependency local

### Requirement 5: Task Execution Across Packages

**User Story:** As a developer, I want to run scripts across all or selected workspace packages, so that I can build, test, and lint my entire monorepo efficiently.

#### Acceptance Criteria

1. WHEN a script command is issued with `--workspace` flag, THE Task_Runner SHALL execute the script in all packages that define it
2. WHEN executing scripts, THE Task_Runner SHALL respect topological order for dependent packages
3. WHEN `--parallel` flag is provided, THE Task_Runner SHALL execute independent packages concurrently
4. WHEN `--filter` pattern is provided, THE Task_Runner SHALL only execute in packages matching the pattern
5. WHEN a script fails in one package, THE Task_Runner SHALL continue or stop based on `--continue-on-error` flag
6. WHEN executing scripts, THE Task_Runner SHALL stream output with package name prefixes for identification
7. WHEN `--since` reference is provided, THE Task_Runner SHALL only execute in packages changed since that reference
8. WHEN a package has no matching script, THE Task_Runner SHALL skip it silently unless `--strict` is specified

### Requirement 6: Change Detection

**User Story:** As a developer, I want to identify which packages have changed, so that I can run targeted builds and tests in CI/CD pipelines.

#### Acceptance Criteria

1. WHEN `--since` is provided with a git ref, THE Change_Detector SHALL identify packages with file changes since that ref
2. WHEN detecting changes, THE Change_Detector SHALL include packages whose dependencies have changed (transitive changes)
3. WHEN a shared configuration file changes (e.g., root tsconfig), THE Change_Detector SHALL mark all affected packages as changed
4. WHEN `.gitignore` patterns exist, THE Change_Detector SHALL exclude ignored files from change detection
5. WHEN queried, THE Change_Detector SHALL return both directly changed and transitively affected packages
6. IF git is not available, THEN THE Change_Detector SHALL return an error indicating git is required

### Requirement 7: Workspace Configuration Validation

**User Story:** As a developer, I want the system to validate my workspace configuration, so that I can catch errors before they cause problems.

#### Acceptance Criteria

1. WHEN a workspace is loaded, THE Workspace_Manager SHALL validate all package.json files against the npm schema
2. WHEN validating, THE Workspace_Manager SHALL check for duplicate package names within the workspace
3. WHEN validating, THE Workspace_Manager SHALL verify all `workspace:` protocol references resolve to existing packages
4. WHEN validating, THE Workspace_Manager SHALL warn about packages not included in any workspace glob
5. IF validation fails, THEN THE Workspace_Manager SHALL return all errors (not just the first) with file locations
6. WHEN `--fix` flag is provided, THE Workspace_Manager SHALL attempt to auto-fix common issues (e.g., missing fields)

### Requirement 8: Version Management

**User Story:** As a developer, I want to manage versions across workspace packages, so that I can release coordinated updates.

#### Acceptance Criteria

1. WHEN a version bump is requested, THE Workspace_Manager SHALL update the specified package's version
2. WHEN a package version changes, THE Workspace_Manager SHALL update all workspace references to that package
3. WHEN `--sync` flag is provided, THE Workspace_Manager SHALL align all package versions to the same value
4. WHEN versioning, THE Workspace_Manager SHALL support semver increment types: major, minor, patch, prerelease
5. WHEN a prerelease tag is specified, THE Workspace_Manager SHALL append it correctly (e.g., `1.0.0-beta.1`)
6. WHEN versions are updated, THE Workspace_Manager SHALL preserve formatting in package.json files

### Requirement 9: Publishing Support

**User Story:** As a developer, I want to publish workspace packages to npm, so that I can share my packages with others.

#### Acceptance Criteria

1. WHEN publishing, THE Workspace_Manager SHALL convert `workspace:*` references to actual version numbers
2. WHEN publishing, THE Workspace_Manager SHALL respect each package's `private` field
3. WHEN publishing with `--dry-run`, THE Workspace_Manager SHALL show what would be published without making changes
4. WHEN publishing, THE Workspace_Manager SHALL publish packages in topological order (dependencies first)
5. IF a package has unpublished workspace dependencies, THEN THE Workspace_Manager SHALL publish dependencies first or return an error
6. WHEN publishing, THE Workspace_Manager SHALL support npm registry authentication via `.npmrc` or environment variables

### Requirement 10: Performance and Caching

**User Story:** As a developer, I want workspace operations to be fast, so that my development workflow is not slowed down.

#### Acceptance Criteria

1. WHEN loading a workspace, THE Workspace_Manager SHALL cache parsed package.json files in memory
2. WHEN the dependency graph is computed, THE Workspace_Manager SHALL cache it until a package.json changes
3. WHEN file watching is enabled, THE Workspace_Manager SHALL invalidate caches incrementally on file changes
4. WHEN performing operations, THE Workspace_Manager SHALL use parallel I/O for independent file operations
5. WHEN serializing workspace state, THE Workspace_Manager SHALL use dx-serializer for binary format efficiency
6. WHEN a cached state exists, THE Workspace_Manager SHALL validate cache freshness before use

### Requirement 11: Integration with DX Ecosystem

**User Story:** As a developer, I want the monorepo system to integrate with other DX tools, so that I have a cohesive development experience.

#### Acceptance Criteria

1. WHEN resolving dependencies, THE Workspace_Manager SHALL delegate to dx-js-package-manager for external packages
2. WHEN building packages, THE Workspace_Manager SHALL integrate with dx-js-bundler for TypeScript/JavaScript compilation
3. WHEN running tests, THE Workspace_Manager SHALL integrate with dx-js-test-runner for test execution
4. WHEN the runtime needs workspace info, THE Workspace_Manager SHALL provide package resolution to dx-js-runtime
5. WHEN configuration is needed, THE Workspace_Manager SHALL support dx.json workspace configuration format
