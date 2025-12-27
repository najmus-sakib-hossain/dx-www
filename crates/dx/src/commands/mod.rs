//! Command implementations for the DX CLI
//!
//! The 9 main DX tools:
//! 1. style      - Binary CSS (B-CSS) compiler
//! 2. media      - Image/video optimization (WebP, AVIF)
//! 3. font       - Font subsetting and WOFF2 optimization
//! 4. icon       - SVG icon system with binary encoding
//! 5. forge      - Package manager + orchestrator
//! 6. serializer - World-record data format (DX ∞)
//! 7. driven     - AI agents control
//! 8. generator  - Code generation tools
//! 9. workspace  - Code editors + preinstall and setup

pub mod driven;
pub mod font;
pub mod forge;
pub mod generator;
pub mod icon;
pub mod media;
pub mod serializer;
pub mod style;
pub mod workspace;
