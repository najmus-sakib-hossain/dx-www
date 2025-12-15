//! Font search functionality
//!
//! Provides unified search across all font providers with optimized
//! concurrent fetching for maximum performance.

use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::models::{FontFamily, FontProvider, SearchQuery, SearchResults, FontCategory};
use crate::providers::ProviderRegistry;
use crate::cdn::{CdnUrlGenerator, FontCdnUrls};

/// Main font search engine with performance optimizations
pub struct FontSearch {
    registry: Arc<ProviderRegistry>,
}

impl FontSearch {
    /// Create a new font search engine with default providers
    pub fn new() -> Result<Self> {
        let registry = ProviderRegistry::with_defaults()?;
        Ok(Self {
            registry: Arc::new(registry),
        })
    }
    
    /// Create with a custom registry
    pub fn with_registry(registry: ProviderRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }
    
    /// Search for fonts matching the query across all providers (concurrent)
    pub async fn search(&self, query: &str) -> Result<SearchResults> {
        let search_query = SearchQuery {
            query: query.to_string(),
            ..Default::default()
        };
        
        self.registry.search_all(&search_query).await
    }
    
    /// Search with timing information
    pub async fn search_timed(&self, query: &str) -> Result<(SearchResults, Duration)> {
        let start = Instant::now();
        let results = self.search(query).await?;
        let elapsed = start.elapsed();
        Ok((results, elapsed))
    }
    
    /// Search with advanced options
    pub async fn search_advanced(&self, query: SearchQuery) -> Result<SearchResults> {
        self.registry.search_all(&query).await
    }
    
    /// Search for fonts by category
    pub async fn search_by_category(&self, category: FontCategory) -> Result<SearchResults> {
        let query = SearchQuery {
            query: String::new(),
            category: Some(category.clone()),
            ..Default::default()
        };
        
        let mut results = self.registry.search_all(&query).await?;
        
        // Filter by category
        results.fonts = results.fonts
            .into_iter()
            .filter(|f| f.category.as_ref() == Some(&category))
            .collect();
        results.total = results.fonts.len();
        
        Ok(results)
    }
    
    /// List all available fonts from all providers (concurrent)
    pub async fn list_all(&self) -> Result<SearchResults> {
        self.registry.list_all_concurrent().await
    }
    
    /// List all with timing information
    pub async fn list_all_timed(&self) -> Result<(SearchResults, Duration)> {
        let start = Instant::now();
        let results = self.list_all().await?;
        let elapsed = start.elapsed();
        Ok((results, elapsed))
    }
    
    /// Get detailed information about a specific font
    pub async fn get_font_details(&self, provider: &FontProvider, font_id: &str) -> Result<FontFamily> {
        for p in self.registry.providers() {
            if p.name() == provider.name() {
                return p.get_font_family(font_id).await;
            }
        }
        
        Err(anyhow::anyhow!("Provider not found: {:?}", provider))
    }
    
    /// Get CDN URLs for a font for preview/usage
    pub fn get_cdn_urls(&self, font_id: &str, font_name: &str, provider: &FontProvider) -> FontCdnUrls {
        match provider {
            FontProvider::GoogleFonts => CdnUrlGenerator::for_google_font(font_id, font_name),
            FontProvider::BunnyFonts => CdnUrlGenerator::for_bunny_font(font_id, font_name),
            FontProvider::Fontsource => CdnUrlGenerator::for_fontsource_font(font_id),
            _ => CdnUrlGenerator::for_google_font(font_id, font_name),
        }
    }
    
    /// Check health of all providers (concurrent)
    pub async fn health_check(&self) -> Vec<(String, bool)> {
        self.registry.health_check_all().await
            .into_iter()
            .map(|(name, healthy, _)| (name, healthy))
            .collect()
    }
    
    /// Check health with timing information
    pub async fn health_check_timed(&self) -> Vec<(String, bool, Duration)> {
        self.registry.health_check_all().await
    }
    
    /// Get statistics about available fonts
    pub async fn get_stats(&self) -> Result<FontStats> {
        let (results, elapsed) = self.list_all_timed().await?;
        
        let mut stats = FontStats::default();
        stats.total_fonts = results.total;
        stats.providers_count = results.providers_searched.len();
        stats.providers = results.providers_searched;
        stats.fetch_time_ms = elapsed.as_millis() as u64;
        
        // Count by category using parallel processing (Rayon)
        let category_counts: Vec<(Option<FontCategory>, usize)> = results.fonts
            .par_iter()
            .fold(
                || std::collections::HashMap::new(),
                |mut acc, font| {
                    *acc.entry(font.category.clone()).or_insert(0) += 1;
                    acc
                }
            )
            .reduce(
                || std::collections::HashMap::new(),
                |mut a, b| {
                    for (k, v) in b {
                        *a.entry(k).or_insert(0) += v;
                    }
                    a
                }
            )
            .into_iter()
            .collect();
        
        for (category, count) in category_counts {
            match category {
                Some(FontCategory::Serif) => stats.serif_count = count,
                Some(FontCategory::SansSerif) => stats.sans_serif_count = count,
                Some(FontCategory::Display) => stats.display_count = count,
                Some(FontCategory::Handwriting) => stats.handwriting_count = count,
                Some(FontCategory::Monospace) => stats.monospace_count = count,
                None => stats.uncategorized_count = count,
            }
        }
        
        Ok(stats)
    }
}

/// Font statistics with performance metrics
#[derive(Debug, Default, serde::Serialize)]
pub struct FontStats {
    pub total_fonts: usize,
    pub providers_count: usize,
    pub providers: Vec<String>,
    pub serif_count: usize,
    pub sans_serif_count: usize,
    pub display_count: usize,
    pub handwriting_count: usize,
    pub monospace_count: usize,
    pub uncategorized_count: usize,
    pub fetch_time_ms: u64,
}
