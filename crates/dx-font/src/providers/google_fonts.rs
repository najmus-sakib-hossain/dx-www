//! Google Fonts provider implementation
//!
//! Google Fonts provides 1,562+ free, open-source fonts.
//! API: https://fonts.google.com

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use crate::models::{Font, FontFamily, FontVariant, FontCategory, FontProvider, FontWeight, FontStyle, FontLicense, SearchQuery};
use super::FontProviderTrait;

/// Google Webfonts Helper API response
#[derive(Debug, Deserialize)]
pub struct GwfhFont {
    pub id: String,
    pub family: String,
    pub variants: Vec<String>,
    pub subsets: Vec<String>,
    pub category: String,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    pub version: Option<String>,
    pub popularity: Option<u32>,
    #[serde(rename = "defSubset")]
    pub default_subset: Option<String>,
}

/// Google Fonts provider using the Google Webfonts Helper API
pub struct GoogleFontsProvider {
    client: Client,
    api_url: String,
}

impl GoogleFontsProvider {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            api_url: "https://gwfh.mranftl.com/api/fonts".to_string(),
        }
    }
    
    fn parse_variant(variant: &str) -> (FontWeight, FontStyle) {
        let is_italic = variant.contains("italic");
        let weight_str = variant.replace("italic", "").replace("regular", "400");
        
        let weight = match weight_str.trim() {
            "" | "400" => FontWeight::Regular,
            "100" => FontWeight::Thin,
            "200" => FontWeight::ExtraLight,
            "300" => FontWeight::Light,
            "500" => FontWeight::Medium,
            "600" => FontWeight::SemiBold,
            "700" => FontWeight::Bold,
            "800" => FontWeight::ExtraBold,
            "900" => FontWeight::Black,
            _ => FontWeight::Regular,
        };
        
        let style = if is_italic { FontStyle::Italic } else { FontStyle::Normal };
        
        (weight, style)
    }
    
    fn parse_category(category: &str) -> Option<FontCategory> {
        match category.to_lowercase().as_str() {
            "serif" => Some(FontCategory::Serif),
            "sans-serif" => Some(FontCategory::SansSerif),
            "display" => Some(FontCategory::Display),
            "handwriting" => Some(FontCategory::Handwriting),
            "monospace" => Some(FontCategory::Monospace),
            _ => None,
        }
    }
}

#[async_trait]
impl FontProviderTrait for GoogleFontsProvider {
    fn name(&self) -> &str {
        "Google Fonts"
    }
    
    fn base_url(&self) -> &str {
        "https://fonts.google.com"
    }
    
    async fn search(&self, query: &SearchQuery) -> Result<Vec<Font>> {
        let fonts = self.list_all().await?;
        
        let query_lower = query.query.to_lowercase();
        let filtered: Vec<Font> = fonts
            .into_iter()
            .filter(|f| f.name.to_lowercase().contains(&query_lower))
            .collect();
        
        Ok(filtered)
    }
    
    async fn list_all(&self) -> Result<Vec<Font>> {
        let response: Vec<GwfhFont> = self.client
            .get(&self.api_url)
            .send()
            .await?
            .json()
            .await?;
        
        let fonts: Vec<Font> = response
            .into_iter()
            .map(|f| Font {
                id: f.id.clone(),
                name: f.family.clone(),
                provider: FontProvider::GoogleFonts,
                category: Self::parse_category(&f.category),
                variant_count: f.variants.len(),
                license: Some(FontLicense::OFL),
                preview_url: Some(format!(
                    "https://fonts.google.com/specimen/{}",
                    f.family.replace(' ', "+")
                )),
                download_url: Some(format!(
                    "https://gwfh.mranftl.com/api/fonts/{}?download=zip&subsets=latin&formats=ttf",
                    f.id
                )),
            })
            .collect();
        
        Ok(fonts)
    }
    
    async fn get_font_family(&self, font_id: &str) -> Result<FontFamily> {
        let url = format!("{}/{}", self.api_url, font_id);
        let response: GwfhFont = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        
        let variants: Vec<FontVariant> = response.variants
            .iter()
            .map(|v| {
                let (weight, style) = Self::parse_variant(v);
                FontVariant {
                    weight,
                    style,
                    file_url: Some(format!(
                        "https://gwfh.mranftl.com/api/fonts/{}/{}?download=zip",
                        font_id, v
                    )),
                    file_format: "ttf".to_string(),
                }
            })
            .collect();
        
        Ok(FontFamily {
            id: response.id,
            name: response.family.clone(),
            provider: FontProvider::GoogleFonts,
            category: Self::parse_category(&response.category),
            variants,
            license: Some(FontLicense::OFL),
            designer: None,
            description: None,
            preview_url: Some(format!(
                "https://fonts.google.com/specimen/{}",
                response.family.replace(' ', "+")
            )),
            download_url: Some(format!(
                "https://gwfh.mranftl.com/api/fonts/{}?download=zip&subsets=latin&formats=ttf",
                font_id
            )),
            languages: vec!["Latin".to_string()],
            subsets: response.subsets,
            popularity: response.popularity,
            last_modified: response.last_modified,
        })
    }
    
    async fn get_download_url(&self, font_id: &str) -> Result<String> {
        Ok(format!(
            "https://gwfh.mranftl.com/api/fonts/{}?download=zip&subsets=latin&formats=ttf,woff2",
            font_id
        ))
    }
    
    async fn health_check(&self) -> Result<bool> {
        let response = self.client
            .head(&self.api_url)
            .send()
            .await?;
        Ok(response.status().is_success())
    }
}
