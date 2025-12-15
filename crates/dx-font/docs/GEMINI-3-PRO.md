Here is the professional **README.md** file for your project. This file is designed to be placed at the root of your repository. It explains the architecture, the ecosystem (excluding dx-icons/fonts), the technology stack, and the usage.

---

# 🚀 Dx Media

> **The Universal Digital Asset Engine**  
> *Query, Scrape, Download, and Process any media format from the free web.*

![Rust](https://img.shields.io/badge/Built_with-Rust-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Active_Development-green?style=for-the-badge)

**Dx Media** is a high-performance, asynchronous CLI tool built in Rust. It acts as a unified interface to the internet's free asset libraries. Whether you need 4K stock footage, massive CSV datasets for AI, full-text novels, or 3D models, Dx Media finds them, downloads them, and processes them into your local workflow.

---

## ⚠️ The Dx Ecosystem Note

**Dx Media** focuses on heavy media and documents. It is part of the broader **Dx Suite**:

*   **Dx Media:** Images, Videos, Audio, Documents, Datasets, 3D Models.
*   **Dx Icons:** (External Project) Specialized vector icon management.
*   **Dx Fonts:** (External Project) Typography and font management.

*This tool explicitly excludes Icon and Font processing to maintain separation of concerns.*

---

## 🌟 Features

*   **Universal Query Interface:** Write one query, target any media type.
*   **Hybrid Engine:** Combines **API integration** (Unsplash, Pexels, GitHub) with **Advanced Web Scraping** (Wikipedia, Toon sites) to find assets even without official APIs.
*   **Smart Processing:**
    *   Converts HTML articles to Markdown/Text automatically.
    *   Handles format conversion (JPG -> PNG, etc.).
    *   Sanitizes filenames and organizes folder structures.
*   **Blazing Fast:** Built on **Tokio**, utilizing async concurrency to download massive files and datasets in parallel.
*   **UX Focused:** Professional progress bars (`indicatif`) for large downloads.

---

## 📦 Supported Media & Providers

| Media Type | Source Strategy | Provider / Tech |
| :--- | :--- | :--- |
| **Images** | API | **Unsplash** (High-Res Photography) |
| **Videos** | API | **Pexels** (HD/4K Stock Video) |
| **Text / Novels** | Web Scraping | **Wikipedia** (Articles parsed to Markdown) |
| **Spreadsheets** | Code Search | **GitHub** (CSVs, Excel sheets via Raw Content) |
| **JSON Data** | Code Search | **GitHub** (Datasets for Machine Learning) |
| **Toons / Comics** | Web Scraping | **XKCD** (Proof of concept scraper) |
| **3D Models** | *In Progress* | *PolyHaven / Sketchfab scraping* |
| **PDF / Docs** | *In Progress* | *Google Custom Search / Filetype filtering* |

---

## 🛠️ Tech Stack (The Best of Rust)

We utilize the latest and most robust crates in the Rust ecosystem (2024/2025 standards):

*   **Core Runtime:** `tokio` (Async Runtime), `futures-util`.
*   **Network:** `reqwest` (HTTP Client with Stream/Multipart support).
*   **CLI Interface:** `clap` (Derive-based argument parsing).
*   **Scraping Engine:**
    *   `scraper`: High-level CSS selector parsing (jQuery-like).
    *   `html2md`: Converts scraped HTML structures into clean Text/Markdown.
*   **Data Handling:**
    *   `serde` & `serde_json`: Serialization standard.
    *   `csv`: Fast CSV parsing.
*   **Media Processing:** `image` (Pixel manipulation).
*   **User Experience:** `indicatif` (Progress bars), `colored`.
*   **Configuration:** `dotenvy` (Environment variable management).

---

## 🚀 Installation & Setup

### 1. Prerequisites
Ensure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone & Build
```bash
git clone https://github.com/yourusername/dx-media.git
cd dx-media
cargo build --release
```

### 3. Configuration (.env)
Create a `.env` file in the root directory to store your API keys for the providers that require them:

```env
# Get free keys from their respective developer portals
UNSPLASH_ACCESS_KEY=your_unsplash_key
PEXELS_API_KEY=your_pexels_key
```

*(Note: Scrapers like Wikipedia and GitHub Data search work without keys, though GitHub has lower rate limits without a token).*

---

## 📖 Usage Examples

The CLI structure is simple: `dx-media --query "search term" --media-type [type]`.

### 1. Download 4K Stock Video
Search for nature videos and save them to `/media/video`.
```bash
cargo run -- --query "mountain river" --media-type video --count 5
```

### 2. Fetch AI Training Data (CSV)
Search GitHub for free datasets related to housing prices.
```bash
cargo run -- --query "housing prices" --media-type spreadsheet
```

### 3. Scrape & Save Articles
Search Wikipedia for a topic, scrape the content, strip the HTML, and save as a Markdown file.
```bash
cargo run -- --query "Artificial Intelligence" --media-type text
```

### 4. Get Webcomics
Download the latest comics.
```bash
cargo run -- --query "all" --media-type toon
```

---

## 🗺️ Roadmap

- [x] **Phase 1:** Project Setup, Image (Unsplash), Basic CLI.
- [x] **Phase 2:** Video (Pexels), Text Scraping (Wiki), Async Downloads.
- [x] **Phase 3:** Data (CSV/JSON via GitHub), Toons, Expanded Models.
- [ ] **Phase 4 (Next):** 
    - 3D Asset Scraping (GLTF/OBJ).
    - PDF Document Search.
    - `ffmpeg` integration for video format conversion (mp4 -> gif/webm).
- [ ] **Phase 5:** Plugin system for user-defined scrapers.

---

## 🤝 Contributing

Contributions are welcome! Please note that this project strictly adheres to the **Dx Suite** boundaries. If you want to add Icon support, please contribute to `dx-icons`.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

---

**Dx Media** — *The internet is your asset library.*