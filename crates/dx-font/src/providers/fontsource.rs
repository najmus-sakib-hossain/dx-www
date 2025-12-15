//! Fontsource provider implementation
//!
//! Fontsource provides 1,562+ open-source fonts as NPM packages.
//! API: https://api.fontsource.org/v1/fonts

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use crate::models::{Font, FontFamily, FontVariant, FontCategory, FontProvider, FontWeight, FontStyle, FontLicense, SearchQuery};
use super::FontProviderTrait;

/// Fontsource API font response
#[derive(Debug, Deserialize)]
pub struct FontsourceFont {
    pub id: String,
    pub family: String,
    pub category: String,
    pub weights: Vec<i32>,
    pub styles: Vec<String>,
    pub subsets: Vec<String>,
    #[serde(rename = "type")]
    pub font_type: Option<String>,
    pub variable: Option<bool>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    pub version: Option<String>,
}

/// Fontsource provider
pub struct FontsourceProvider {
    client: Client,
    api_url: String,
}

impl FontsourceProvider {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            api_url: "https://api.fontsource.org/v1/fonts".to_string(),
        }
    }
    
    fn parse_category(category: &str) -> Option<FontCategory> {
        match category.to_lowercase().as_str() {
            "serif" => Some(FontCategory::Serif),
            "sans-serif" | "sans serif" => Some(FontCategory::SansSerif),
            "display" => Some(FontCategory::Display),
            "handwriting" => Some(FontCategory::Handwriting),
            "monospace" | "mono" => Some(FontCategory::Monospace),
            _ => None,
        }
    }
}

#[async_trait]
impl FontProviderTrait for FontsourceProvider {
    fn name(&self) -> &str {
        "Fontsource"
    }
    
    fn base_url(&self) -> &str {
        "https://fontsource.org"
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
        let response: Vec<FontsourceFont> = self.client
            .get(&self.api_url)
            .send()
            .await?
            .json()
            .await?;
        
        let fonts: Vec<Font> = response
            .into_iter()
            .map(|f| {
                let variant_count = f.weights.len() * f.styles.len();
                Font {
                    id: f.id.clone(),
                    name: f.family.clone(),
                    provider: FontProvider::Fontsource,
                    category: Self::parse_category(&f.category),
                    variant_count,
                    license: Some(FontLicense::OFL),
                    preview_url: Some(format!(
                        "https://fontsource.org/fonts/{}",
                        f.id
                    )),
                    download_url: Some(format!(
                        "https://cdn.jsdelivr.net/npm/@fontsource/{}/files",
                        f.id
                    )),
                }
            })
            .collect();
        
        Ok(fonts)
    }
    
    async fn get_font_family(&self, font_id: &str) -> Result<FontFamily> {
        let url = format!("{}/{}", self.api_url, font_id);
        let response: FontsourceFont = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        
        let mut variants = Vec::new();
        for weight in &response.weights {
            for style in &response.styles {
                let font_weight = FontWeight::from_numeric(*weight as u16);
                let font_style = if style == "italic" {
                    FontStyle::Italic
                } else {
                    FontStyle::Normal
                };
                
                variants.push(FontVariant {
                    weight: font_weight,
                    style: font_style,
                    file_url: Some(format!(
                        "https://cdn.jsdelivr.net/npm/@fontsource/{}/files/{}-{}-latin-{}.woff2",
                        response.id, response.id, weight, style
                    )),
                    file_format: "woff2".to_string(),
                });
            }
        }
        
        Ok(FontFamily {
            id: response.id.clone(),
            name: response.family.clone(),
            provider: FontProvider::Fontsource,
            category: Self::parse_category(&response.category),
            variants,
            license: Some(FontLicense::OFL),
            designer: None,
            description: None,
            preview_url: Some(format!(
                "https://fontsource.org/fonts/{}",
                response.id
            )),
            download_url: Some(format!(
                "https://cdn.jsdelivr.net/npm/@fontsource/{}/files",
                response.id
            )),
            languages: vec!["Latin".to_string()],
            subsets: response.subsets,
            popularity: None,
            last_modified: response.last_modified,
        })
    }
    
    async fn get_download_url(&self, font_id: &str) -> Result<String> {
        // For Fontsource, we use jsdelivr CDN for direct file access
        Ok(format!(
            "https://cdn.jsdelivr.net/npm/@fontsource/{}/files",
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
