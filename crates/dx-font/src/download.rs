//! Font download functionality
//!
//! Handles downloading fonts from various providers with progress indication.

use anyhow::{Result, Context};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::models::{FontProvider, DownloadOptions};
use crate::providers::{create_http_client, ProviderRegistry};

/// Font downloader with progress indication
pub struct FontDownloader {
    client: Client,
    registry: Arc<ProviderRegistry>,
    multi_progress: MultiProgress,
}

impl FontDownloader {
    /// Create a new font downloader
    pub fn new() -> Result<Self> {
        let client = create_http_client()?;
        let registry = ProviderRegistry::with_defaults()?;
        
        Ok(Self {
            client,
            registry: Arc::new(registry),
            multi_progress: MultiProgress::new(),
        })
    }
    
    /// Download a font by ID from a specific provider
    pub async fn download_font(
        &self,
        provider: &FontProvider,
        font_id: &str,
        options: &DownloadOptions,
    ) -> Result<Vec<PathBuf>> {
        // Ensure output directory exists
        fs::create_dir_all(&options.output_dir).await?;
        
        // Get download URL from provider
        let download_url = self.get_download_url(provider, font_id).await?;
        
        // Download the font
        self.download_file(&download_url, &options.output_dir, font_id).await
    }
    
    /// Download a font using a direct URL
    pub async fn download_from_url(
        &self,
        url: &str,
        output_dir: &Path,
        filename: &str,
    ) -> Result<PathBuf> {
        fs::create_dir_all(output_dir).await?;
        
        let paths = self.download_file(url, output_dir, filename).await?;
        paths.into_iter().next().ok_or_else(|| anyhow::anyhow!("No files downloaded"))
    }
    
    /// Download using Google Webfonts Helper (provides zip with all formats)
    pub async fn download_google_font(
        &self,
        font_id: &str,
        output_dir: &Path,
        formats: &[&str],
        subsets: &[&str],
    ) -> Result<PathBuf> {
        fs::create_dir_all(output_dir).await?;
        
        let formats_str = formats.join(",");
        let subsets_str = subsets.join(",");
        
        let url = format!(
            "https://gwfh.mranftl.com/api/fonts/{}?download=zip&subsets={}&formats={}",
            font_id, subsets_str, formats_str
        );
        
        let output_path = output_dir.join(format!("{}.zip", font_id));
        
        let pb = self.create_progress_bar(font_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download font: HTTP {}",
                response.status()
            ));
        }
        
        let total_size = response.content_length().unwrap_or(0);
        pb.set_length(total_size);
        
        let mut file = File::create(&output_path).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Error reading chunk")?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }
        
        pb.finish_with_message(format!("Downloaded {}", font_id));
        
        Ok(output_path)
    }
    
    /// Download font from Fontsource via CDN
    pub async fn download_fontsource_font(
        &self,
        font_id: &str,
        output_dir: &Path,
        weight: u16,
        style: &str,
    ) -> Result<PathBuf> {
        fs::create_dir_all(output_dir).await?;
        
        let url = format!(
            "https://cdn.jsdelivr.net/npm/@fontsource/{}/files/{}-latin-{}-{}.woff2",
            font_id, font_id, weight, style
        );
        
        let filename = format!("{}-{}-{}.woff2", font_id, weight, style);
        let output_path = output_dir.join(&filename);
        
        let pb = self.create_progress_bar(&filename);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;
        
        if !response.status().is_success() {
            pb.finish_with_message(format!("Failed: {}", filename));
            return Err(anyhow::anyhow!(
                "Failed to download font: HTTP {}",
                response.status()
            ));
        }
        
        let total_size = response.content_length().unwrap_or(0);
        pb.set_length(total_size);
        
        let mut file = File::create(&output_path).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Error reading chunk")?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }
        
        pb.finish_with_message(format!("Downloaded {}", filename));
        
        Ok(output_path)
    }
    
    async fn get_download_url(&self, provider: &FontProvider, font_id: &str) -> Result<String> {
        for p in self.registry.providers() {
            if p.name() == provider.name() {
                return p.get_download_url(font_id).await;
            }
        }
        
        Err(anyhow::anyhow!("Provider not found: {:?}", provider))
    }
    
    async fn download_file(
        &self,
        url: &str,
        output_dir: &Path,
        name: &str,
    ) -> Result<Vec<PathBuf>> {
        let pb = self.create_progress_bar(name);
        
        let response = self.client
            .get(url)
            .send()
            .await
            .context("Failed to send request")?;
        
        if !response.status().is_success() {
            pb.finish_with_message(format!("Failed: {}", name));
            return Err(anyhow::anyhow!(
                "Failed to download: HTTP {}",
                response.status()
            ));
        }
        
        // Determine file extension from content-type or URL
        let extension = self.get_extension_from_response(&response, url);
        let filename = format!("{}.{}", name, extension);
        let output_path = output_dir.join(&filename);
        
        let total_size = response.content_length().unwrap_or(0);
        pb.set_length(total_size);
        
        let mut file = File::create(&output_path).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Error reading chunk")?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }
        
        pb.finish_with_message(format!("Downloaded {}", filename));
        
        Ok(vec![output_path])
    }
    
    fn create_progress_bar(&self, name: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(0));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        pb.set_message(format!("Downloading {}", name));
        pb
    }
    
    fn get_extension_from_response(&self, response: &reqwest::Response, url: &str) -> String {
        // Try to get from content-type
        if let Some(content_type) = response.headers().get("content-type") {
            if let Ok(ct) = content_type.to_str() {
                if ct.contains("zip") {
                    return "zip".to_string();
                } else if ct.contains("woff2") {
                    return "woff2".to_string();
                } else if ct.contains("woff") {
                    return "woff".to_string();
                } else if ct.contains("ttf") || ct.contains("truetype") {
                    return "ttf".to_string();
                } else if ct.contains("otf") || ct.contains("opentype") {
                    return "otf".to_string();
                }
            }
        }
        
        // Try to get from URL
        if url.contains(".zip") {
            "zip".to_string()
        } else if url.contains(".woff2") {
            "woff2".to_string()
        } else if url.contains(".woff") {
            "woff".to_string()
        } else if url.contains(".ttf") {
            "ttf".to_string()
        } else if url.contains(".otf") {
            "otf".to_string()
        } else {
            "zip".to_string() // Default to zip for font packages
        }
    }
}

/// Download result
#[derive(Debug)]
pub struct DownloadResult {
    pub font_id: String,
    pub provider: FontProvider,
    pub files: Vec<PathBuf>,
    pub success: bool,
    pub error: Option<String>,
}
