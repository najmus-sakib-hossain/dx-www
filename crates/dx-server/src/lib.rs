//! # dx-server: The Holographic Server
//!
//! High-performance SSR & Edge Runtime for dx-www
//!
//! **Role:** Serve Binary Snapshots, Handle SSR Inflation (SEO), Manage State
//! **Philosophy:** "Write TSX, Serve Binary"
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────┐
//! │  TSX Files   │
//! └──────┬───────┘
//!        ↓
//! ┌──────────────┐
//! │ dx-compiler  │
//! └──────┬───────┘
//!        ↓
//! ┌──────────────┐
//! │  .dxb Files  │ ← Binary Format
//! └──────┬───────┘
//!        ↓
//! ┌──────────────┐
//! │  dx-server   │ ← YOU ARE HERE
//! └──────┬───────┘
//!        ↓
//! ┌──────────────┐
//! │   Browser    │
//! │  (dx-cache)  │
//! └──────────────┘
//! ```

pub mod delta;
mod handlers;
mod ssr;
pub mod stream;

// Ecosystem integrations
pub mod ecosystem;

// Ecosystem handlers (feature-gated)
#[cfg(feature = "query")]
pub mod rpc_handler;

#[cfg(feature = "auth")]
pub mod auth_middleware;

#[cfg(feature = "sync")]
pub mod ws_handler;

pub use handlers::*;
pub use ssr::*;

use axum::{routing::get, Router};

use dashmap::DashMap;
use dx_packet::Template;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

/// Global server state
#[derive(Clone)]
pub struct ServerState {
    /// Binary snapshot cache (path -> bytes)
    pub binary_cache: Arc<DashMap<String, Vec<u8>>>,
    /// Template cache (id -> Template) - stores full Template structs
    pub template_cache: Arc<DashMap<u32, Template>>,
    /// Version storage for delta patching (hash -> binary data)
    /// Stores last 5 versions of each artifact for patch generation
    pub version_store: Arc<std::sync::Mutex<delta::VersionStore>>,
    /// Current version hash (artifact name -> hash)
    pub current_version: Arc<DashMap<String, String>>,
    /// Project directory for serving static files (index.html, etc.)
    pub project_dir: Arc<std::sync::RwLock<Option<std::path::PathBuf>>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            binary_cache: Arc::new(DashMap::new()),
            template_cache: Arc::new(DashMap::new()),
            version_store: Arc::new(std::sync::Mutex::new(delta::VersionStore::new(5))),
            current_version: Arc::new(DashMap::new()),
            project_dir: Arc::new(std::sync::RwLock::new(None)),
        }
    }

    /// Load artifacts from build output directory
    pub fn load_artifacts(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("📦 Loading artifacts from {}", path.display());

        // Load templates.json (parsed templates)
        let templates_path = path.join("templates.json");
        if templates_path.exists() {
            let json_str = std::fs::read_to_string(&templates_path)?;
            let templates: Vec<Template> = serde_json::from_str(&json_str)?;

            tracing::info!("  ✓ Loaded {} templates", templates.len());

            // Populate cache with full Template structs
            for template in templates {
                self.template_cache.insert(template.id, template);
            }
        } else {
            tracing::warn!("  ⚠️ templates.json not found");
        }

        // Load layout.bin (raw binary for streaming)
        let layout_path = path.join("layout.bin");
        if layout_path.exists() {
            let bytes = std::fs::read(&layout_path)?;
            self.binary_cache.insert("layout.bin".to_string(), bytes.clone());

            // Store version for delta patching
            let hash = {
                let mut store = self.version_store.lock().unwrap();
                store.store(bytes)
            };
            self.current_version.insert("layout.bin".to_string(), hash.clone());
            tracing::debug!(
                "  ✓ Cached layout.bin ({} bytes, hash: {})",
                self.binary_cache.get("layout.bin").unwrap().len(),
                &hash[..8]
            );
        }

        // Load app.wasm
        let wasm_path = path.join("app.wasm");
        if wasm_path.exists() {
            let bytes = std::fs::read(&wasm_path)?;
            let size = bytes.len();
            self.binary_cache.insert("app.wasm".to_string(), bytes.clone());

            // Store version for delta patching
            let hash = {
                let mut store = self.version_store.lock().unwrap();
                store.store(bytes)
            };
            self.current_version.insert("app.wasm".to_string(), hash.clone());
            tracing::info!("  ✓ Loaded app.wasm ({} bytes, hash: {})", size, &hash[..8]);
        }

        Ok(())
    }

    /// Register a template manually (for testing or dynamic loading)
    pub fn register_template(&self, template: Template) {
        let id = template.id;
        self.template_cache.insert(id, template);
        tracing::debug!("📄 Registered template {}", id);
    }

    /// Set project directory for serving static files
    pub fn set_project_dir(&self, dir: std::path::PathBuf) {
        tracing::info!("📁 Project directory: {}", dir.display());
        *self.project_dir.write().unwrap() = Some(dir);
    }
}

/// Build the Axum router with all routes
pub fn build_router(state: ServerState) -> Router {
    let mut router = Router::new()
        // Root index (supports bot detection + SSR)
        .route("/", get(handlers::serve_index))
        // Health check
        .route("/health", get(handlers::health_check))
        // Favicon (prevent 404)
        .route("/favicon.ico", get(handlers::serve_favicon))
        // Binary streaming endpoint (Day 16: The Binary Streamer)
        .route("/stream/:app_id", get(handlers::serve_binary_stream));

    // Add ecosystem routes
    #[cfg(feature = "query")]
    {
        use axum::routing::post;
        router = router.route("/api/rpc", post(rpc_handler::handle_rpc));
    }

    #[cfg(feature = "auth")]
    {
        use axum::routing::post;
        router = router.route("/api/auth/login", post(auth_middleware::handle_login));
    }

    #[cfg(feature = "sync")]
    {
        router = router.route("/ws", get(ws_handler::handle_ws_upgrade));
    }

    router
        // Add state
        .with_state(state)
        // Middleware
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

/// Start the dx-server
pub async fn serve(addr: SocketAddr, state: ServerState) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("🚀 dx-server starting at {}", addr);

    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("✨ dx-server ready - The Holographic Server is online");
    tracing::info!("📦 Binary streaming enabled");
    tracing::info!("🔍 SEO inflation ready");
    tracing::info!("⚡ Delta patching active");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let state = ServerState::new();
        assert_eq!(state.binary_cache.len(), 0);
        assert_eq!(state.template_cache.len(), 0);
    }
}
