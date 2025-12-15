//! Font provider implementations
//!
//! This module contains implementations for various font providers/sources.
//! Optimized for performance with concurrent fetching and connection pooling.

pub mod google_fonts;
pub mod bunny_fonts;
pub mod fontsource;
pub mod fontshare;
pub mod font_library;
pub mod github_fonts;
pub mod dafont;
pub mod fontspace;
pub mod fonts1001;
pub mod fontsquirrel;

use async_trait::async_trait;
use crate::models::{FontFamily, Font, SearchQuery, SearchResults};
use anyhow::Result;
use futures::future::join_all;
use tokio::time::{timeout, Duration, Instant};
use std::sync::Arc;

/// Trait that all font providers must implement
#[async_trait]
pub trait FontProviderTrait: Send + Sync {
    /// Get the name of this provider
    fn name(&self) -> &str;
    
    /// Get the base URL for this provider
    fn base_url(&self) -> &str;
    
    /// Search for fonts matching the query
    async fn search(&self, query: &SearchQuery) -> Result<Vec<Font>>;
    
    /// Get all available fonts from this provider
    async fn list_all(&self) -> Result<Vec<Font>>;
    
    /// Get detailed information about a specific font family
    async fn get_font_family(&self, font_id: &str) -> Result<FontFamily>;
    
    /// Get download URL for a font
    async fn get_download_url(&self, font_id: &str) -> Result<String>;
    
    /// Check if the provider is available/responding
    async fn health_check(&self) -> Result<bool>;
}

/// Create an HTTP client with optimized settings for performance
pub fn create_http_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .user_agent(format!("dx-font/{}", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        // Connection pooling for better performance
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        // Enable compression for faster transfers
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .build()?;
    Ok(client)
}

/// Create a fast HTTP client with shorter timeouts for health checks
pub fn create_fast_http_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .user_agent(format!("dx-font/{}", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(3))
        .pool_max_idle_per_host(5)
        .build()?;
    Ok(client)
}

/// Registry of all available font providers
pub struct ProviderRegistry {
    providers: Vec<Arc<dyn FontProviderTrait>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }
    
    pub fn with_defaults() -> Result<Self> {
        let client = create_http_client()?;
        let mut registry = Self::new();
        
        // Add all providers for maximum font coverage (50k+ fonts)
        registry.register(Arc::new(google_fonts::GoogleFontsProvider::new(client.clone())));
        registry.register(Arc::new(bunny_fonts::BunnyFontsProvider::new(client.clone())));
        registry.register(Arc::new(fontsource::FontsourceProvider::new(client.clone())));
        registry.register(Arc::new(fontshare::FontshareProvider::new(client.clone())));
        registry.register(Arc::new(font_library::FontLibraryProvider::new(client.clone())));
        registry.register(Arc::new(github_fonts::GitHubFontsProvider::new(client.clone())));
        registry.register(Arc::new(dafont::DafontProvider::new(client.clone())));
        registry.register(Arc::new(fontspace::FontSpaceProvider::new(client.clone())));
        registry.register(Arc::new(fonts1001::Fonts1001Provider::new(client.clone())));
        registry.register(Arc::new(fontsquirrel::FontSquirrelProvider::new(client.clone())));
        
        Ok(registry)
    }
    
    pub fn register(&mut self, provider: Arc<dyn FontProviderTrait>) {
        self.providers.push(provider);
    }
    
    pub fn providers(&self) -> &[Arc<dyn FontProviderTrait>] {
        &self.providers
    }
    
    /// Search all providers concurrently for maximum speed
    pub async fn search_all(&self, query: &SearchQuery) -> Result<SearchResults> {
        let start = Instant::now();
        let providers_searched: Vec<String> = self.providers.iter()
            .map(|p| p.name().to_string())
            .collect();
        
        // Create search futures for all providers
        let search_futures: Vec<_> = self.providers.iter()
            .map(|provider| {
                let provider = Arc::clone(provider);
                let query = query.clone();
                async move {
                    // Add timeout for each provider to prevent slow providers from blocking
                    match timeout(Duration::from_secs(15), provider.search(&query)).await {
                        Ok(Ok(fonts)) => {
                            tracing::debug!("Provider {} returned {} fonts", provider.name(), fonts.len());
                            fonts
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Error searching {}: {}", provider.name(), e);
                            Vec::new()
                        }
                        Err(_) => {
                            tracing::warn!("Timeout searching {}", provider.name());
                            Vec::new()
                        }
                    }
                }
            })
            .collect();
        
        // Execute all searches concurrently
        let results = join_all(search_futures).await;
        
        // Flatten results from all providers
        let mut all_fonts: Vec<Font> = results.into_iter().flatten().collect();
        
        let total = all_fonts.len();
        
        // Apply limit if specified
        if let Some(limit) = query.limit {
            all_fonts.truncate(limit);
        }
        
        let elapsed = start.elapsed();
        tracing::info!("Search completed in {:?}, found {} fonts", elapsed, total);
        
        Ok(SearchResults {
            fonts: all_fonts,
            total,
            query: query.query.clone(),
            providers_searched,
        })
    }
    
    /// List all fonts from all providers concurrently
    pub async fn list_all_concurrent(&self) -> Result<SearchResults> {
        let start = Instant::now();
        let providers_searched: Vec<String> = self.providers.iter()
            .map(|p| p.name().to_string())
            .collect();
        
        // Create list futures for all providers
        let list_futures: Vec<_> = self.providers.iter()
            .map(|provider| {
                let provider = Arc::clone(provider);
                async move {
                    match timeout(Duration::from_secs(30), provider.list_all()).await {
                        Ok(Ok(fonts)) => {
                            tracing::debug!("Provider {} listed {} fonts", provider.name(), fonts.len());
                            fonts
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Error listing from {}: {}", provider.name(), e);
                            Vec::new()
                        }
                        Err(_) => {
                            tracing::warn!("Timeout listing from {}", provider.name());
                            Vec::new()
                        }
                    }
                }
            })
            .collect();
        
        // Execute all lists concurrently
        let results = join_all(list_futures).await;
        let all_fonts: Vec<Font> = results.into_iter().flatten().collect();
        let total = all_fonts.len();
        
        let elapsed = start.elapsed();
        tracing::info!("List all completed in {:?}, found {} fonts", elapsed, total);
        
        Ok(SearchResults {
            fonts: all_fonts,
            total,
            query: String::new(),
            providers_searched,
        })
    }
    
    /// Check health of all providers concurrently
    pub async fn health_check_all(&self) -> Vec<(String, bool, Duration)> {
        let health_futures: Vec<_> = self.providers.iter()
            .map(|provider| {
                let provider = Arc::clone(provider);
                async move {
                    let start = Instant::now();
                    let is_healthy = match timeout(Duration::from_secs(5), provider.health_check()).await {
                        Ok(Ok(healthy)) => healthy,
                        _ => false,
                    };
                    let elapsed = start.elapsed();
                    (provider.name().to_string(), is_healthy, elapsed)
                }
            })
            .collect();
        
        join_all(health_futures).await
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
