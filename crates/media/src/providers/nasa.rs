//! NASA Images provider implementation.
//!
//! [NASA Images API Documentation](https://images.nasa.gov/docs/images.nasa.gov_api_docs.pdf)
//!
//! Provides access to NASA's image and video library with 140,000+ public domain assets.

use async_trait::async_trait;
use serde::Deserialize;
use std::time::Duration;

use crate::config::Config;
use crate::error::Result;
use crate::http::{HttpClient, ResponseExt};
use crate::providers::traits::{Provider, ProviderInfo};
use crate::types::{License, MediaAsset, MediaType, RateLimitConfig, SearchQuery, SearchResult};

/// NASA Images provider for space and science media.
/// Access to 140K+ public domain images and videos.
#[derive(Debug)]
pub struct NasaImagesProvider {
    client: HttpClient,
}

impl NasaImagesProvider {
    /// Create a new NASA Images provider.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        let client = HttpClient::with_config(
            Self::RATE_LIMIT,
            config.retry_attempts,
            Duration::from_secs(config.timeout_secs),
        )
        .unwrap_or_default();

        Self { client }
    }

    /// Rate limit: Unlimited (but be respectful)
    const RATE_LIMIT: RateLimitConfig = RateLimitConfig::new(1000, 3600);

    /// Get the media type filter string for the API
    fn media_type_filter(media_type: Option<MediaType>) -> &'static str {
        match media_type {
            Some(MediaType::Image) => "image",
            Some(MediaType::Video) => "video",
            Some(MediaType::Audio) => "audio",
            _ => "image",
        }
    }

    /// Parse media type from string
    fn parse_media_type(s: &str) -> MediaType {
        match s {
            "video" => MediaType::Video,
            "audio" => MediaType::Audio,
            _ => MediaType::Image,
        }
    }
}

#[async_trait]
impl Provider for NasaImagesProvider {
    fn name(&self) -> &'static str {
        "nasa"
    }

    fn display_name(&self) -> &'static str {
        "NASA Images"
    }

    fn supported_media_types(&self) -> &[MediaType] {
        &[MediaType::Image, MediaType::Video, MediaType::Audio]
    }

    fn requires_api_key(&self) -> bool {
        false
    }

    fn rate_limit(&self) -> RateLimitConfig {
        Self::RATE_LIMIT
    }

    fn is_available(&self) -> bool {
        true
    }

    fn base_url(&self) -> &'static str {
        "https://images-api.nasa.gov"
    }

    async fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        let url = format!("{}/search", self.base_url());

        let media_type = Self::media_type_filter(query.media_type);
        let page_str = query.page.to_string();
        let count_str = query.count.min(100).to_string();

        let params = [
            ("q", query.query.as_str()),
            ("media_type", media_type),
            ("page", &page_str),
            ("page_size", &count_str),
        ];

        let response = self.client.get_with_query(&url, &params, &[]).await?;

        let api_response: NasaSearchResponse = response.json_or_error().await?;

        let assets: Vec<MediaAsset> = api_response
            .collection
            .items
            .into_iter()
            .filter_map(|item| {
                let data = item.data.into_iter().next()?;
                let link = item.links.and_then(|l| l.into_iter().next());

                let preview_url = link.as_ref().map(|l| l.href.clone());

                // NASA assets are all public domain
                let asset = MediaAsset::builder()
                    .id(data.nasa_id)
                    .provider("nasa")
                    .media_type(Self::parse_media_type(&data.media_type))
                    .title(data.title)
                    .download_url(preview_url.clone().unwrap_or_default())
                    .preview_url(preview_url.unwrap_or_default())
                    .source_url(item.href)
                    .author(data.center.unwrap_or_else(|| "NASA".to_string()))
                    .license(License::PublicDomain)
                    .tags(data.keywords.unwrap_or_default())
                    .build();

                Some(asset)
            })
            .collect();

        Ok(SearchResult {
            query: query.query.clone(),
            media_type: query.media_type,
            total_count: api_response.collection.metadata.total_hits,
            assets,
            providers_searched: vec!["nasa".to_string()],
            provider_errors: vec![],
            duration_ms: 0,
        })
    }
}

impl ProviderInfo for NasaImagesProvider {
    fn description(&self) -> &'static str {
        "NASA's official image and video library with space and science media"
    }

    fn api_key_url(&self) -> &'static str {
        "https://images.nasa.gov/docs/images.nasa.gov_api_docs.pdf"
    }

    fn default_license(&self) -> &'static str {
        "Public Domain (NASA media is not copyrighted)"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// API RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NasaSearchResponse {
    collection: NasaCollection,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NasaCollection {
    items: Vec<NasaItem>,
    metadata: NasaMetadata,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NasaMetadata {
    total_hits: usize,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NasaItem {
    href: String,
    data: Vec<NasaItemData>,
    links: Option<Vec<NasaLink>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NasaItemData {
    nasa_id: String,
    title: String,
    media_type: String,
    description: Option<String>,
    center: Option<String>,
    date_created: Option<String>,
    #[serde(default)]
    keywords: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NasaLink {
    href: String,
    rel: String,
    render: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_metadata() {
        let config = Config::default_for_testing();
        let provider = NasaImagesProvider::new(&config);

        assert_eq!(provider.name(), "nasa");
        assert_eq!(provider.display_name(), "NASA Images");
        assert!(!provider.requires_api_key());
        assert!(provider.is_available());
    }

    #[test]
    fn test_supported_media_types() {
        let config = Config::default_for_testing();
        let provider = NasaImagesProvider::new(&config);

        let types = provider.supported_media_types();
        assert!(types.contains(&MediaType::Image));
        assert!(types.contains(&MediaType::Video));
        assert!(types.contains(&MediaType::Audio));
    }
}
