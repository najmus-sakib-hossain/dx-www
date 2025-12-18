# Requirements Document

## Introduction

This document specifies the Binary Dawn architecture update for dx-forge, transforming it from a VCS into a Sovereign Orchestration Engine. The update introduces three critical subsystems: Sovereign Orchestrator (tool lifecycle management), Shadow Worker (background tasks), and Traffic Branching System (revolutionary package management).

## Glossary

- **DxForge**: The main entry point struct that coordinates all subsystems
- **Sovereign Orchestrator**: System managing lifecycle (init, run, watch, kill) of all dx- tools
- **Shadow Worker**: Background task processor for caching, R2 syncing, and pre-fetching
- **Traffic Branching System**: Package manager replacement that injects dependencies into source code
- **TrafficLight**: Safety signal (Green/Yellow/Red) for dependency injection decisions
- **BackgroundTask**: Enumeration of tasks that can be processed asynchronously
- **DxToolDefinition**: Configuration struct defining a controlled DX tool

## Requirements

### Requirement 1

**User Story:** As a developer, I want a unified DxForge entry point, so that I can access all subsystems through a single interface.

#### Acceptance Criteria

1. WHEN DxForge is initialized THEN the system SHALL create instances of Orchestrator, BackgroundWorker, and TrafficManager
2. WHEN a pipeline command is executed THEN the system SHALL coordinate between subsystems appropriately
3. WHEN the "install" command runs THEN the system SHALL invoke TrafficManager and enqueue background caching
4. WHEN the "build" command runs THEN the system SHALL ensure dependent tools are running before execution

### Requirement 2

**User Story:** As a developer, I want the Sovereign Orchestrator to manage tool lifecycles, so that I can have single-point control over all dx- tools.

#### Acceptance Criteria

1. WHEN a tool is registered THEN the Orchestrator SHALL store its definition with name, binary path, priority, and dependencies
2. WHEN ensure_running is called for a stopped tool THEN the Orchestrator SHALL start the tool and update its status to Running
3. WHEN ensure_running is called for an already running tool THEN the Orchestrator SHALL return immediately without restarting
4. WHEN execute_tool is called THEN the Orchestrator SHALL run the specified tool with provided arguments
5. WHEN a tool status is queried THEN the Orchestrator SHALL return one of: Stopped, Starting, Running(PID), Healthy, or Degraded

### Requirement 3

**User Story:** As a developer, I want background tasks to run without blocking my terminal, so that heavy operations like R2 syncing happen asynchronously.

#### Acceptance Criteria

1. WHEN BackgroundWorker is created THEN the system SHALL spawn an async task listening for background tasks
2. WHEN CacheCurrentState task is enqueued THEN the worker SHALL snapshot the codebase state for rollback
3. WHEN SyncToCloudflareR2 task is enqueued THEN the worker SHALL upload artifacts to R2 bucket
4. WHEN PrefetchPackage task is enqueued THEN the worker SHALL download the specified package for future use
5. WHEN a task is enqueued THEN the system SHALL return control to the caller immediately

### Requirement 4

**User Story:** As a developer, I want the Traffic Branching System to inject dependencies into my source tree, so that I have full visibility and control over dependency code.

#### Acceptance Criteria

1. WHEN install_package is called THEN the system SHALL analyze traffic safety for each file
2. WHEN a file does not exist THEN the system SHALL return TrafficLight::Green and allow injection
3. WHEN a file exists with identical content THEN the system SHALL return TrafficLight::Green
4. WHEN a file exists with different content THEN the system SHALL return TrafficLight::Yellow for merge
5. WHEN install_all_dependencies is called THEN the system SHALL process all dependencies from configuration
6. WHEN TrafficLight is Green THEN the system SHALL inject the file directly
7. WHEN TrafficLight is Yellow THEN the system SHALL perform merge logic
8. WHEN TrafficLight is Red THEN the system SHALL preserve user version and create .new file for comparison

### Requirement 5

**User Story:** As a developer, I want to run complete pipelines through DxForge, so that I can orchestrate complex workflows with a single command.

#### Acceptance Criteria

1. WHEN run_pipeline("install") is called THEN the system SHALL install dependencies and enqueue caching
2. WHEN run_pipeline("build") is called THEN the system SHALL ensure tools are running, execute bundler, and sync to R2
3. WHEN an unknown pipeline command is provided THEN the system SHALL log an appropriate message
