//! Core Forge functionality
//!
//! This module contains the main `Forge` struct that provides a unified API
//! for DX tools to manage their lifecycle, version control, and code generation.

pub mod editor_integration;
pub mod forge;
pub mod lifecycle;
pub mod tracking;

pub use editor_integration::{EditorInfo, EditorIntegration, EditorType, OutputStrategy};
pub use forge::{Forge, ForgeConfig};
pub use lifecycle::{LifecycleEvent, LifecycleManager, ToolId, ToolState, ToolStatus};
pub use tracking::{GeneratedCodeTracker, GeneratedFileInfo};
