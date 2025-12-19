//! Command implementations for the DX CLI
//!
//! The 10 main DX tools:
//! 1. style      - Binary CSS (B-CSS) compiler
//! 2. media      - Image/video optimization (WebP, AVIF)
//! 3. font       - Font subsetting and WOFF2 optimization
//! 4. icon       - SVG icon system with binary encoding
//! 5. forge      - Package manager + orchestrator
//! 6. serializer - World-record data format (DX ∞)
//! 7. stack      - JS/TS runtime, bundler, test runner, package manager
//! 8. driven     - AI agents control
//! 9. generator  - Code generation tools
//! 10. workspace - Code editors + preinstall and setup

pub mod driven;
pub mod font;
pub mod forge;
pub mod generator;
pub mod icon;
pub mod media;
pub mod serializer;
pub mod stack;
pub mod style;
pub mod workspace;
