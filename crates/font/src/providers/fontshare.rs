//! FontShare provider implementation
//!
//! FontShare is a curated collection of 100+ professional, commercial-free fonts.
//! API: https://www.fontshare.com

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use crate::models::{Font, FontFamily, FontVariant, FontCategory, FontProvider, FontWeight, FontStyle, FontLicense, SearchQuery};
use super::FontProviderTrait;

/// FontShare API font response
#[derive(Debug, Deserialize)]
pub struct FontshareResponse {
    pub fonts: Vec<FontshareFont>,
}

#[derive(Debug, Deserialize)]
pub struct FontshareFont {
    pub slug: String,
    pub name: String,
    pub styles: Vec<FontshareStyle>,
    pub category: Option<String>,
    pub designer: Option<FontshareDesigner>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FontshareStyle {
    pub name: String,
    pub weight: Option<i32>,
    pub is_italic: Option<bool>,
    pub file: Option<FontshareFile>,
}

#[derive(Debug, Deserialize)]
pub struct FontshareFile {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FontshareDesigner {
    pub name: Option<String>,
}

/// FontShare provider
pub struct FontshareProvider {
    client: Client,
    api_url: String,
}

impl FontshareProvider {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            api_url: "https://api.fontshare.com/v2/fonts".to_string(),
        }
    }
    
    fn parse_category(category: &Option<String>) -> Option<FontCategory> {
        category.as_ref().and_then(|c| {
            match c.to_lowercase().as_str() {
                "serif" => Some(FontCategory::Serif),
                "sans-serif" | "sans" => Some(FontCategory::SansSerif),
                "display" => Some(FontCategory::Display),
                "handwriting" | "script" => Some(FontCategory::Handwriting),
                "monospace" | "mono" => Some(FontCategory::Monospace),
                _ => None,
            }
        })
    }
}

#[async_trait]
impl FontProviderTrait for FontshareProvider {
    fn name(&self) -> &str {
        "FontShare"
    }
    
    fn base_url(&self) -> &str {
        "https://www.fontshare.com"
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
        // FontShare API might require different handling
        // For now, we'll try the basic API endpoint
        let response = self.client
            .get(&self.api_url)
            .send()
            .await?;
        
        // Check if we get a valid response
        if !response.status().is_success() {
            // Return empty if API is not accessible
            return Ok(Vec::new());
        }
        
        let text = response.text().await?;
        
        // Try to parse the response
        match serde_json::from_str::<FontshareResponse>(&text) {
            Ok(data) => {
                let fonts: Vec<Font> = data.fonts
                    .into_iter()
                    .map(|f| Font {
                        id: f.slug.clone(),
                        name: f.name.clone(),
                        provider: FontProvider::FontShare,
                        category: Self::parse_category(&f.category),
                        variant_count: f.styles.len(),
                        license: Some(FontLicense::FreeCommercial),
                        preview_url: Some(format!(
                            "https://www.fontshare.com/fonts/{}",
                            f.slug
                        )),
                        download_url: Some(format!(
                            "https://www.fontshare.com/fonts/{}/download",
                            f.slug
                        )),
                    })
                    .collect();
                
                Ok(fonts)
            }
            Err(_) => {
                // If parsing fails, try to parse as an array directly
                match serde_json::from_str::<Vec<FontshareFont>>(&text) {
                    Ok(fonts_vec) => {
                        let fonts: Vec<Font> = fonts_vec
                            .into_iter()
                            .map(|f| Font {
                                id: f.slug.clone(),
                                name: f.name.clone(),
                                provider: FontProvider::FontShare,
                                category: Self::parse_category(&f.category),
                                variant_count: f.styles.len(),
                                license: Some(FontLicense::FreeCommercial),
                                preview_url: Some(format!(
                                    "https://www.fontshare.com/fonts/{}",
                                    f.slug
                                )),
                                download_url: Some(format!(
                                    "https://www.fontshare.com/fonts/{}/download",
                                    f.slug
                                )),
                            })
                            .collect();
                        
                        Ok(fonts)
                    }
                    Err(_) => Ok(Vec::new()),
                }
            }
        }
    }
    
    async fn get_font_family(&self, font_id: &str) -> Result<FontFamily> {
        let url = format!("{}/{}", self.api_url, font_id);
        let response: FontshareFont = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        
        let variants: Vec<FontVariant> = response.styles
            .iter()
            .map(|s| {
                let weight = s.weight
                    .map(|w| FontWeight::from_numeric(w as u16))
                    .unwrap_or(FontWeight::Regular);
                let style = if s.is_italic.unwrap_or(false) {
                    FontStyle::Italic
                } else {
                    FontStyle::Normal
                };
                
                FontVariant {
                    weight,
                    style,
                    file_url: s.file.as_ref().and_then(|f| f.url.clone()),
                    file_format: "ttf".to_string(),
                }
            })
            .collect();
        
        Ok(FontFamily {
            id: response.slug.clone(),
            name: response.name.clone(),
            provider: FontProvider::FontShare,
            category: Self::parse_category(&response.category),
            variants,
            license: Some(FontLicense::FreeCommercial),
            designer: response.designer.and_then(|d| d.name),
            description: response.description,
            preview_url: Some(format!(
                "https://www.fontshare.com/fonts/{}",
                response.slug
            )),
            download_url: Some(format!(
                "https://www.fontshare.com/fonts/{}/download",
                response.slug
            )),
            languages: vec!["Latin".to_string()],
            subsets: vec!["latin".to_string()],
            popularity: None,
            last_modified: None,
        })
    }
    
    async fn get_download_url(&self, font_id: &str) -> Result<String> {
        Ok(format!(
            "https://www.fontshare.com/fonts/{}/download",
            font_id
        ))
    }
    
    async fn health_check(&self) -> Result<bool> {
        let response = self.client
            .head("https://www.fontshare.com")
            .send()
            .await?;
        Ok(response.status().is_success())
    }
}
