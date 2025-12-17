# 🎬 DX Media

<div align="center">

```
██████╗ ██╗  ██╗    ███╗   ███╗███████╗██████╗ ██╗ █████╗ 
██╔══██╗╚██╗██╔╝    ████╗ ████║██╔════╝██╔══██╗██║██╔══██╗
██║  ██║ ╚███╔╝     ██╔████╔██║█████╗  ██║  ██║██║███████║
██║  ██║ ██╔██╗     ██║╚██╔╝██║██╔══╝  ██║  ██║██║██╔══██║
██████╔╝██╔╝ ██╗    ██║ ╚═╝ ██║███████╗██████╔╝██║██║  ██║
╚═════╝ ╚═╝  ╚═╝    ╚═╝     ╚═╝╚══════╝╚═════╝ ╚═╝╚═╝  ╚═╝
                                                          
        ═══════════════════════════════════════════
              THE UNIVERSAL DIGITAL ASSET ENGINE
        ═══════════════════════════════════════════
```

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Runtime-Tokio-blue?style=for-the-badge&logo=rust&logoColor=white)](https://tokio.rs/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-brightgreen?style=for-the-badge)](CONTRIBUTING.md)
[![Downloads](https://img.shields.io/badge/Downloads-Coming_Soon-yellow?style=for-the-badge)]()

**One Command. Any Media. From Anywhere.**

*The Swiss Army Knife for Digital Asset Acquisition*

[Why DX Media?](#-why-dx-media-matters) •
[Features](#-features) •
[Providers](#-50-free-api-providers) •
[Installation](#-installation) •
[Usage](#-usage) •
[Roadmap](#-roadmap)

---

### 🌟 Star us on GitHub — it motivates us a lot!

</div>

---

## 🎯 Why DX Media Matters

### The Problem Every Developer Faces

```
Developer: "I need a sunset image for my app"
Reality:   
  → Google "free sunset images"
  → Open 10 tabs (Unsplash, Pexels, Pixabay...)
  → Create accounts, get API keys
  → Learn each API's quirks
  → Download manually
  → Rename files
  → Organize into folders
  → Repeat for videos, audio, data...

Time wasted: 2+ hours
Frustration level: 💀💀💀💀💀
```

### The DX Media Solution

```bash
dx search "sunset" --type image --download --count 10
```

```
✅ Found 847 results across 5 providers
⬇️  Downloading 10 assets...
████████████████████████████████████████ 100%

📁 Saved to ./media/images/
   ├── sunset_beach_unsplash_abc123.jpg
   ├── sunset_mountains_pexels_def456.jpg
   └── ... (8 more)

Time: 4.2 seconds
```

---

## 🚀 Why This Project is AWESOME for Modern Development

### 1. 🏗️ **Perfect for Modern Tech Stacks**

| Use Case | How DX Media Helps |
|----------|-------------------|
| **AI/ML Projects** | Instantly download training datasets, labeled images, CSV data |
| **Web Development** | Stock photos, videos, icons, placeholder content |
| **Game Development** | Sprites, tilesets, sound effects, 3D models |
| **Content Creation** | Royalty-free music, B-roll footage, graphics |
| **Documentation** | Diagrams, screenshots, technical illustrations |
| **Prototyping** | Rapid asset acquisition for MVPs |
| **Education** | Textbooks, research papers, educational videos |
| **Data Science** | Datasets from Kaggle, GitHub, government portals |

### 2. 💰 **Massive Cost Savings**

```
Traditional Asset Acquisition:
├── Stock Photo Subscription:     $29-299/month
├── Music Licensing:              $15-500/track
├── Video Footage:                $79-500/clip
├── 3D Model Marketplaces:        $10-1000/model
├── Dataset Purchases:            $100-10000/set
└── Developer Time:               $50-200/hour
                                  ───────────────
                                  $$$$ EXPENSIVE

DX Media Approach:
├── Software:                     FREE (Open Source)
├── API Access:                   FREE (Free Tiers)
├── Assets:                       FREE (CC0, Public Domain)
└── Developer Time:               MINIMAL (Automated)
                                  ───────────────
                                  $0 - Priceless
```

### 3. ⚡ **Developer Experience Revolution**

```rust
// Old Way (100+ lines of boilerplate per provider)
let client = reqwest::Client::new();
let response = client
    .get("https://api.unsplash.com/search/photos")
    .header("Authorization", format!("Client-ID {}", api_key))
    .query(&[("query", "nature"), ("per_page", "10")])
    .send()
    .await?;
let data: UnsplashResponse = response.json().await?;
// ... parse, download, save, organize...

// DX Media Way (1 line)
let assets = dx.search("nature", MediaType::Image).await?;
```

### 4. 🔄 **CI/CD & Automation Ready**

```yaml
# GitHub Actions Example
- name: Fetch Assets for Build
  run: |
    dx search "hero background" --type image --count 1 --download
    dx search "notification" --type soundeffect --count 5 --download
    dx search "loading" --type animation --count 3 --download
```

### 5. 🌍 **Ethical & Legal by Default**

- ✅ All providers offer **free, legal content**
- ✅ Proper **attribution tracking** built-in
- ✅ **License metadata** preserved
- ✅ No piracy, no gray areas

### 6. 🧩 **Ecosystem Integration**

```
DX Media integrates with everything:

┌─────────────────────────────────────────────────────────────┐
│                      YOUR PROJECT                           │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────────┐ │
│  │ React   │  │ Unity   │  │ Python  │  │ Rust/Go/Node    │ │
│  │ Vue     │  │ Unreal  │  │ Jupyter │  │ Any Language    │ │
│  │ Svelte  │  │ Godot   │  │ ML/AI   │  │ Any Framework   │ │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────────┬────────┘ │
│       │            │            │                │          │
│       └────────────┴────────────┴────────────────┘          │
│                            │                                │
│                     ┌──────▼──────┐                         │
│                     │  DX MEDIA   │                         │
│                     │   ./media   │                         │
│                     └─────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

---

## ✨ Features

### Core Capabilities

| Feature | Description |
|---------|-------------|
| **🔍 Universal Search** | One query syntax for 50+ providers |
| **📥 Smart Downloads** | Async, parallel, resumable |
| **🔄 Format Conversion** | Convert anything to anything |
| **📁 Auto-Organization** | Intelligent file structure |
| **⚡ Blazing Fast** | Built on Tokio async runtime |
| **🛡️ Rate Limit Handling** | Automatic throttling per provider |
| **💾 Intelligent Caching** | Avoid redundant downloads |
| **🔌 Plugin System** | Extend with custom providers |
| **🖥️ Beautiful CLI** | Progress bars, colors, interactive mode |
| **📚 Library Mode** | Use as a Rust crate in your projects |

### Media Type Support

```
┌────────────────────────────────────────────────────────────────────┐
│                    150+ NATIVE MEDIA TYPES                         │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  🖼️  VISUAL          🎬 VIDEO           🎵 AUDIO                   │
│  ───────────────    ───────────────    ───────────────            │
│  • Images           • Stock Footage    • Music                     │
│  • Photos           • Animations       • Sound Effects             │
│  • Illustrations    • GIFs             • Podcasts                  │
│  • Vectors          • Tutorials        • Audiobooks                │
│  • Wallpapers       • Time-lapses      • Ambient                   │
│  • Screenshots      • VFX              • Voice                     │
│  • Memes            • Transitions      • Loops                     │
│                                                                    │
│  📄 TEXT             📁 DOCUMENTS       📊 DATA                     │
│  ───────────────    ───────────────    ───────────────            │
│  • Articles         • PDFs             • JSON                      │
│  • Books            • Word Docs        • CSV / TSV                 │
│  • E-books          • Excel            • XML / YAML                │
│  • Research Papers  • PowerPoint       • Datasets                  │
│  • Documentation    • Spreadsheets     • SQL Dumps                 │
│  • Quotes           • Templates        • Parquet                   │
│                                                                    │
│  🎨 3D ASSETS        💻 CODE            🎮 GAME ASSETS              │
│  ───────────────    ───────────────    ───────────────            │
│  • 3D Models        • Snippets         • Sprites                   │
│  • Textures         • Templates        • Tilesets                  │
│  • Materials        • Boilerplates     • UI Kits                   │
│  • HDRIs            • Algorithms       • Characters                │
│  • Skyboxes         • Configs          • Backgrounds               │
│  • Animations       • Scripts          • Particles                 │
│                                                                    │
│  🎭 CREATIVE         📦 ARCHIVES        🔧 OTHER                    │
│  ───────────────    ───────────────    ───────────────            │
│  • Mockups          • ZIP              • Emojis                    │
│  • Wireframes       • Tarballs         • Stickers                  │
│  • Color Palettes   • Packages         • Cursors                   │
│  • Logos            • Bundles          • Themes                    │
│  • Comics/Toons     • Backups          • And more...               │
│                                                                    │
├────────────────────────────────────────────────────────────────────┤
│  🕷️  SCRAPER MODE: ∞ UNLIMITED FILE TYPES FROM ANY WEBSITE        │
└────────────────────────────────────────────────────────────────────┘
```

---

## 🔌 50+ Free API Providers

### 🖼️ Image Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Unsplash](https://unsplash.com/developers)** | High-res photography | ✅ Required | 50/hour | Unsplash License |
| **[Pexels](https://www.pexels.com/api/)** | Stock photos & videos | ✅ Required | 200/hour | Pexels License |
| **[Pixabay](https://pixabay.com/api/docs/)** | Photos, vectors, videos | ✅ Required | Unlimited | Pixabay License |
| **[Lorem Picsum](https://picsum.photos/)** | Random placeholder images | ❌ None | Unlimited | Unsplash License |
| **[Placeholder.com](https://placeholder.com/)** | Custom placeholder images | ❌ None | Unlimited | Free |
| **[RandomUser](https://randomuser.me/)** | Random avatar photos | ❌ None | Unlimited | Free |
| **[Lorem Faces](https://loremfaces.com/)** | AI-generated faces | ❌ None | Varies | Free |
| **[Picsum](https://picsum.photos/)** | Random images | ❌ None | Unlimited | Various |
| **[PlaceKitten](https://placekitten.com/)** | Cat placeholder images | ❌ None | Unlimited | Free |
| **[Dog API](https://dog.ceo/dog-api/)** | Random dog images | ❌ None | Unlimited | Free |
| **[Cat API](https://thecatapi.com/)** | Random cat images | ⚠️ Optional | 10000/month | Free |
| **[Foodish](https://foodish-api.herokuapp.com/)** | Food images | ❌ None | Unlimited | Free |
| **[Coffee](https://coffee.alexflipnote.dev/)** | Coffee images | ❌ None | Unlimited | Free |
| **[xkcd](https://xkcd.com/json.html)** | Webcomics | ❌ None | Unlimited | CC BY-NC 2.5 |
| **[NASA Images](https://api.nasa.gov/)** | Space photography | ⚠️ Optional | 1000/hour | Public Domain |
| **[Metropolitan Museum](https://metmuseum.github.io/)** | Art images | ❌ None | 80/sec | CC0 |
| **[Art Institute Chicago](https://api.artic.edu/docs/)** | Artwork images | ❌ None | Unlimited | CC0 |
| **[Rijksmuseum](https://data.rijksmuseum.nl/)** | Dutch art | ✅ Required | Unlimited | CC0 |
| **[Harvard Art Museums](https://harvardartmuseums.org/collections/api)** | Art collection | ✅ Required | 2500/day | Various |
| **[Europeana](https://pro.europeana.eu/page/apis)** | European cultural heritage | ✅ Required | Varies | Various |

### 🎬 Video Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Pexels Videos](https://www.pexels.com/api/)** | HD/4K stock footage | ✅ Required | 200/hour | Pexels License |
| **[Pixabay Videos](https://pixabay.com/api/docs/)** | Stock videos | ✅ Required | Unlimited | Pixabay License |
| **[Coverr](https://coverr.co/)** | Beautiful free videos | ❌ None | Unlimited | Coverr License |
| **[Mixkit](https://mixkit.co/)** | Stock video clips | ❌ None | Unlimited | Mixkit License |
| **[Life of Vids](https://lifeofvids.com/)** | HD video clips | ❌ None | Unlimited | CC0 |
| **[Videvo](https://www.videvo.net/stock-video-footage/)** | Stock footage | ❌ None | Varies | Various |
| **[Dareful](https://dareful.com/)** | 4K stock footage | ❌ None | Unlimited | CC BY 4.0 |
| **[Vidsplay](https://www.vidsplay.com/)** | Free stock videos | ❌ None | Unlimited | Free |
| **[NASA Video Gallery](https://images.nasa.gov/)** | Space videos | ❌ None | Unlimited | Public Domain |
| **[Internet Archive Video](https://archive.org/)** | Historical videos | ❌ None | Unlimited | Various |

### 🎵 Audio Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Freesound](https://freesound.org/docs/api/)** | Sound effects & samples | ✅ Required | 60/min | CC Various |
| **[Jamendo](https://developer.jamendo.com/)** | Royalty-free music | ✅ Required | Varies | CC Various |
| **[Free Music Archive](https://freemusicarchive.org/)** | Music tracks | ❌ None | Unlimited | CC Various |
| **[ccMixter](http://ccmixter.org/api)** | Remixable music | ❌ None | Unlimited | CC Various |
| **[Incompetech](https://incompetech.com/)** | Royalty-free music | ❌ None | Unlimited | CC BY |
| **[Bensound](https://www.bensound.com/)** | Royalty-free music | ❌ None | Unlimited | Bensound License |
| **[Purple Planet](https://www.purple-planet.com/)** | Background music | ❌ None | Unlimited | PP License |
| **[BBC Sound Effects](https://sound-effects.bbcrewind.co.uk/)** | SFX library | ❌ None | Unlimited | RemArc License |
| **[SoundBible](https://soundbible.com/)** | Sound effects | ❌ None | Unlimited | Various |
| **[ZapSplat](https://www.zapsplat.com/)** | Sound effects | ⚠️ Optional | Varies | ZapSplat License |
| **[Mixkit Sounds](https://mixkit.co/free-sound-effects/)** | Sound effects | ❌ None | Unlimited | Mixkit License |
| **[LibriVox](https://librivox.org/)** | Audiobooks | ❌ None | Unlimited | Public Domain |
| **[Loyal Books](http://www.loyalbooks.com/)** | Free audiobooks | ❌ None | Unlimited | Public Domain |

### 📄 Text & Books Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Wikipedia](https://www.mediawiki.org/wiki/API)** | Encyclopedia articles | ❌ None | 200/sec | CC BY-SA |
| **[Project Gutenberg](https://www.gutenberg.org/)** | Public domain books | ❌ None | Unlimited | Public Domain |
| **[Open Library](https://openlibrary.org/developers/api)** | Books & covers | ❌ None | Unlimited | Various |
| **[Internet Archive](https://archive.org/developers/)** | Books & documents | ❌ None | Unlimited | Various |
| **[arXiv](https://arxiv.org/help/api)** | Research papers | ❌ None | 1/3sec | Various |
| **[CORE](https://core.ac.uk/services/api)** | Research papers | ✅ Required | Varies | Various |
| **[Semantic Scholar](https://api.semanticscholar.org/)** | Academic papers | ⚠️ Optional | 100/5min | Various |
| **[PubMed](https://www.ncbi.nlm.nih.gov/home/develop/api/)** | Medical literature | ❌ None | 3/sec | Various |
| **[Crossref](https://www.crossref.org/documentation/)** | Scholarly metadata | ❌ None | 50/sec | Various |
| **[OpenAlex](https://docs.openalex.org/)** | Academic works | ❌ None | 100000/day | CC0 |
| **[Quotable](https://github.com/lukePeavey/quotable)** | Famous quotes | ❌ None | 180/min | MIT |
| **[Poem DB](https://github.com/thundercomb/poetrydb)** | Poetry collection | ❌ None | Unlimited | Public Domain |
| **[Standard Ebooks](https://standardebooks.org/)** | High-quality ebooks | ❌ None | Unlimited | Public Domain |
| **[ManyBooks](https://manybooks.net/)** | Free ebooks | ❌ None | Unlimited | Various |

### 📁 Document Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Internet Archive](https://archive.org/)** | PDFs, documents | ❌ None | Unlimited | Various |
| **[Google Drive Templates](https://developers.google.com/drive)** | Doc templates | ✅ Required | Varies | Various |
| **[LibreOffice Templates](https://extensions.libreoffice.org/)** | Office templates | ❌ None | Unlimited | Various |
| **[SlideShare](https://www.slideshare.net/)** | Presentations | Scraper | N/A | Various |
| **[Scribd](https://www.scribd.com/)** | Documents | Scraper | N/A | Various |
| **[PDF Drive](https://www.pdfdrive.com/)** | PDF books | Scraper | N/A | Various |
| **[DocDroid](https://www.docdroid.net/)** | Shared documents | ❌ None | Unlimited | User Upload |
| **[Calameo](https://www.calameo.com/)** | Digital publications | ❌ None | Unlimited | User Upload |

### 📊 Data & Dataset Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[GitHub](https://docs.github.com/rest)** | Code, datasets, JSON, CSV | ⚠️ Optional | 60→5000/hour | Various |
| **[Kaggle](https://www.kaggle.com/docs/api)** | ML datasets | ✅ Required | Varies | Various |
| **[Data.gov](https://www.data.gov/developers/apis)** | US Government data | ❌ None | Unlimited | Public Domain |
| **[Data.gov.uk](https://data.gov.uk/)** | UK Government data | ❌ None | Unlimited | OGL |
| **[EU Open Data](https://data.europa.eu/)** | European data | ❌ None | Unlimited | Various |
| **[World Bank](https://datahelpdesk.worldbank.org/knowledgebase/topics/125589)** | Economic data | ❌ None | Unlimited | CC BY 4.0 |
| **[CKAN Portals](https://ckan.org/)** | Various datasets | ❌ None | Varies | Various |
| **[Socrata](https://dev.socrata.com/)** | Open data | ⚠️ Optional | Varies | Various |
| **[OpenDataSoft](https://data.opendatasoft.com/)** | Global datasets | ❌ None | Varies | Various |
| **[JSONPlaceholder](https://jsonplaceholder.typicode.com/)** | Fake JSON API | ❌ None | Unlimited | Free |
| **[DummyJSON](https://dummyjson.com/)** | Fake JSON data | ❌ None | Unlimited | Free |
| **[MockAPI](https://mockapi.io/)** | Mock data | ⚠️ Optional | Varies | Free |
| **[Random Data API](https://random-data-api.com/)** | Random data | ❌ None | Unlimited | Free |
| **[Faker API](https://fakerapi.it/)** | Fake data | ❌ None | Unlimited | Free |

### 🎨 3D Assets Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Sketchfab](https://sketchfab.com/developers)** | 3D models | ✅ Required | Varies | Various |
| **[Poly Pizza](https://poly.pizza/)** | Low-poly 3D models | ❌ None | Unlimited | CC0 |
| **[PolyHaven](https://polyhaven.com/)** | HDRIs, Textures, Models | ❌ None | Unlimited | CC0 |
| **[Kenney](https://kenney.nl/)** | Game-ready 3D assets | ❌ None | Unlimited | CC0 |
| **[Quaternius](https://quaternius.com/)** | Low-poly models | ❌ None | Unlimited | CC0 |
| **[Turbosquid Free](https://www.turbosquid.com/Search/3D-Models/free)** | 3D models | ⚠️ Optional | Varies | Various |
| **[CGTrader Free](https://www.cgtrader.com/free-3d-models)** | 3D models | ⚠️ Optional | Varies | Various |
| **[Free3D](https://free3d.com/)** | 3D models | ❌ None | Unlimited | Various |
| **[Clara.io](https://clara.io/)** | 3D models | ⚠️ Optional | Varies | Various |
| **[Blendswap](https://blendswap.com/)** | Blender models | ❌ None | Unlimited | CC Various |
| **[OpenGameArt 3D](https://opengameart.org/)** | Game 3D assets | ❌ None | Unlimited | CC Various |
| **[Textures.com](https://www.textures.com/)** | PBR textures | ⚠️ Optional | 15/day free | Textures License |
| **[AmbientCG](https://ambientcg.com/)** | PBR materials | ❌ None | Unlimited | CC0 |
| **[Texture Haven](https://texturehaven.com/)** | Textures | ❌ None | Unlimited | CC0 |

### 💻 Code & Templates Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[GitHub Gists](https://docs.github.com/rest/gists)** | Code snippets | ⚠️ Optional | 60→5000/hour | Various |
| **[GitLab Snippets](https://docs.gitlab.com/ee/api/snippets.html)** | Code snippets | ⚠️ Optional | Varies | Various |
| **[CodePen](https://blog.codepen.io/documentation/api/)** | Frontend snippets | ⚠️ Optional | Varies | Various |
| **[StackOverflow](https://api.stackexchange.com/)** | Code Q&A | ❌ None | 300/day | CC BY-SA |
| **[Dev.to](https://developers.forem.com/api)** | Dev articles | ⚠️ Optional | Varies | Various |
| **[Hashnode](https://api.hashnode.com/)** | Dev blogs | ⚠️ Optional | Varies | Various |
| **[RapidAPI](https://rapidapi.com/)** | API marketplace | ⚠️ Optional | Varies | Various |
| **[Public APIs](https://api.publicapis.org/)** | API directory | ❌ None | 10/min | MIT |
| **[DevDocs](https://devdocs.io/)** | Documentation | ❌ None | Unlimited | Various |
| **[Awesome Lists](https://github.com/sindresorhus/awesome)** | Curated lists | ❌ None | Unlimited | CC0 |

### 🎮 Game Assets Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[OpenGameArt](https://opengameart.org/)** | 2D/3D game assets | ❌ None | Unlimited | CC Various |
| **[Itch.io Assets](https://itch.io/game-assets/free)** | Game assets | ❌ None | Unlimited | Various |
| **[Kenney.nl](https://kenney.nl/assets)** | Game assets | ❌ None | Unlimited | CC0 |
| **[GameArt2D](https://www.gameart2d.com/)** | 2D game graphics | ❌ None | Unlimited | Various |
| **[CraftPix Free](https://craftpix.net/freebies/)** | Game graphics | ❌ None | Unlimited | CraftPix License |
| **[Game-Icons.net](https://game-icons.net/)** | Game icons (via dx-icons) | ❌ None | Unlimited | CC BY 3.0 |
| **[Reiner's Tilesets](https://www.reinerstilesets.de/)** | Sprite sheets | ❌ None | Unlimited | Free |
| **[WidgetWorx](http://www.widgetworx.com/spritelib/)** | Sprite library | ❌ None | Unlimited | CPL |
| **[LPC](https://lpc.opengameart.org/)** | Character generator | ❌ None | Unlimited | CC BY-SA/GPL |
| **[GameDev Market Free](https://www.gamedevmarket.net/)** | Various game assets | ❌ None | Varies | Various |

### 🌐 Miscellaneous Providers

| Provider | Description | API Key | Rate Limit | License |
|----------|-------------|---------|------------|---------|
| **[Giphy](https://developers.giphy.com/)** | GIFs & Stickers | ✅ Required | Varies | Giphy Terms |
| **[Tenor](https://tenor.com/gifapi)** | GIF search | ✅ Required | Varies | Tenor Terms |
| **[Lorem Ipsum](https://loripsum.net/)** | Placeholder text | ❌ None | Unlimited | Free |
| **[Bacon Ipsum](https://baconipsum.com/api/)** | Meaty filler text | ❌ None | Unlimited | Free |
| **[Cupcake Ipsum](http://www.cupcakeipsum.com/)** | Sweet filler text | ❌ None | Unlimited | Free |
| **[Bob Ross Lipsum](https://bobrosslipsum.com/)** | Happy filler text | ❌ None | Unlimited | Free |
| **[Emoji API](https://emoji-api.com/)** | Emoji data | ✅ Required | Varies | Free |
| **[Open Emoji](https://github.com/nickshanks/open-emoji)** | Emoji images | ❌ None | Unlimited | CC0 |
| **[Country Flags](https://flagcdn.com/)** | Flag images | ❌ None | Unlimited | Public Domain |
| **[RestCountries](https://restcountries.com/)** | Country data | ❌ None | Unlimited | MPL 2.0 |
| **[IP Geolocation](https://ipapi.co/)** | Location data | ⚠️ Optional | 1000/day | Free Tier |

---

## 🏗️ The DX Ecosystem

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         DX SUITE ARCHITECTURE                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐             │
│    │  DX MEDIA   │     │  DX ICONS   │     │  DX FONTS   │             │
│    │             │     │             │     │             │             │
│    │  • Images   │     │  • SVG      │     │  • TTF/OTF  │             │
│    │  • Videos   │     │  • Icon     │     │  • WOFF     │             │
│    │  • Audio    │     │    Packs    │     │  • Variable │             │
│    │  • Docs     │     │  • Sprites  │     │  • Web      │             │
│    │  • Data     │     │             │     │             │             │
│    │  • 3D       │     │             │     │             │             │
│    │  • Code     │     │   SEPARATE  │     │   SEPARATE  │             │
│    │  • Games    │     │   PROJECT   │     │   PROJECT   │             │
│    │  • +140     │     │             │     │             │             │
│    └──────┬──────┘     └─────────────┘     └─────────────┘             │
│           │                                                             │
│           ▼                                                             │
│    ┌─────────────────────────────────────────────┐                     │
│    │              YOUR APPLICATION               │                     │
│    │                                             │                     │
│    │   dx search "hero" --type image             │                     │
│    │   dx-icons search "arrow" --style solid     │                     │
│    │   dx-fonts search "sans-serif" --weight 400 │                     │
│    │                                             │                     │
│    └─────────────────────────────────────────────┘                     │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

> ⚠️ **Note:** DX Media focuses on heavy media. Icons and Fonts have dedicated tools for optimal management.

---

## 📦 Installation

### Quick Install

```bash
# Clone the repository
git clone https://github.com/anthropics/dx-media.git
cd dx-media

# Build & Install
cargo install --path .

# Verify installation
dx --version
```

### Add All Dependencies (Latest Crates)

```bash
# ═══════════════════════════════════════════════════════════════
# CORE DEPENDENCIES
# ═══════════════════════════════════════════════════════════════

# Async Runtime
cargo add tokio --features full
cargo add futures
cargo add futures-util
cargo add async-trait

# HTTP & Networking
cargo add reqwest --features "json stream multipart cookies rustls-tls gzip brotli deflate"
cargo add url
cargo add urlencoding
cargo add percent-encoding

# Serialization & Data Formats
cargo add serde --features derive
cargo add serde_json
cargo add serde_yaml
cargo add toml
cargo add csv
cargo add quick-xml --features serialize
cargo add calamine  # Excel/ODS reading

# CLI & Terminal
cargo add clap --features "derive env wrap_help color"
cargo add ratatui
cargo add crossterm
cargo add indicatif
cargo add console
cargo add dialoguer
cargo add colored
cargo add owo-colors

# Error Handling
cargo add thiserror
cargo add anyhow

# Logging
cargo add tracing
cargo add tracing-subscriber --features "env-filter json"

# ═══════════════════════════════════════════════════════════════
# FILE & MEDIA PROCESSING
# ═══════════════════════════════════════════════════════════════

# File System
cargo add walkdir
cargo add glob
cargo add tempfile
cargo add directories
cargo add sanitize-filename

# Type Detection
cargo add mime_guess
cargo add infer
cargo add tree_magic_mini

# Image Processing
cargo add image
cargo add webp
cargo add imageproc
cargo add fast_image_resize

# PDF Processing
cargo add lopdf
cargo add pdf-extract

# Archive Handling
cargo add zip
cargo add flate2
cargo add tar
cargo add bzip2
cargo add xz2

# ═══════════════════════════════════════════════════════════════
# WEB SCRAPING
# ═══════════════════════════════════════════════════════════════

cargo add scraper
cargo add select
cargo add html2text
cargo add pulldown-cmark
cargo add regex
cargo add lazy_static
cargo add ego-tree

# ═══════════════════════════════════════════════════════════════
# UTILITIES
# ═══════════════════════════════════════════════════════════════

cargo add uuid --features v4
cargo add chrono --features serde
cargo add humansize
cargo add humantime
cargo add sha2
cargo add md5
cargo add hex
cargo add base64
cargo add rand
cargo add once_cell
cargo add parking_lot
cargo add dashmap
cargo add bytes
cargo add memmap2

# Rate Limiting & Retry
cargo add governor
cargo add tokio-retry
cargo add backoff --features tokio

# Derive & Macros
cargo add strum --features derive
cargo add derive_more --features full
cargo add bon
cargo add typed-builder

# Environment
cargo add dotenvy
cargo add config

# ═══════════════════════════════════════════════════════════════
# DEV DEPENDENCIES
# ═══════════════════════════════════════════════════════════════

cargo add --dev wiremock
cargo add --dev tokio-test
cargo add --dev assert_fs
cargo add --dev predicates
cargo add --dev pretty_assertions
cargo add --dev mockall
cargo add --dev criterion
cargo add --dev fake --features derive
cargo add --dev proptest
```

### Configuration (.env)

```bash
# Copy example and configure
cp .env.example .env

# Edit with your API keys
nano .env
```

```env
# ═══════════════════════════════════════════════════════════════
# DX MEDIA CONFIGURATION
# ═══════════════════════════════════════════════════════════════

# ─── Directories ───────────────────────────────────────────────
DX_MEDIA_DIR=./media
DX_CACHE_DIR=./cache
DX_TEMP_DIR=./temp

# ─── Image Providers ───────────────────────────────────────────
UNSPLASH_ACCESS_KEY=
PEXELS_API_KEY=
PIXABAY_API_KEY=
NASA_API_KEY=
RIJKSMUSEUM_API_KEY=

# ─── Audio Providers ───────────────────────────────────────────
FREESOUND_API_KEY=
JAMENDO_CLIENT_ID=

# ─── 3D Providers ──────────────────────────────────────────────
SKETCHFAB_API_KEY=

# ─── Data Providers ────────────────────────────────────────────
GITHUB_TOKEN=
KAGGLE_USERNAME=
KAGGLE_KEY=

# ─── GIF Providers ─────────────────────────────────────────────
GIPHY_API_KEY=
TENOR_API_KEY=

# ─── Settings ──────────────────────────────────────────────────
DX_CONCURRENT_DOWNLOADS=5
DX_RETRY_ATTEMPTS=3
DX_TIMEOUT_SECONDS=300
DX_RESPECT_RATE_LIMITS=true
DX_CACHE_ENABLED=true
DX_CACHE_TTL_HOURS=24
```

---

## 🖥️ Usage

### CLI Examples

```bash
# ═══════════════════════════════════════════════════════════════
# 🔍 SEARCH
# ═══════════════════════════════════════════════════════════════

# Search images
dx search "mountain sunset" --type image

# Search with provider filter
dx search "ocean" --type video --provider pexels

# Search with count
dx search "electronic" --type music --count 20

# Search datasets
dx search "housing prices" --type dataset

# Search research papers
dx search "transformer neural network" --type paper

# Search 3D models
dx search "low poly tree" --type model3d

# Search with multiple types
dx search "cat" --type image,gif,video

# ═══════════════════════════════════════════════════════════════
# ⬇️  DOWNLOAD
# ═══════════════════════════════════════════════════════════════

# Download search results
dx search "forest" --type image --download

# Download specific URL
dx download "https://unsplash.com/photos/abc123"

# Download to custom directory
dx download "https://example.com/dataset.csv" --output ./data

# Batch download from file
dx download --from urls.txt

# Download with conversion
dx download "https://example.com/image.png" --convert webp

# ═══════════════════════════════════════════════════════════════
# 🔄 CONVERT
# ═══════════════════════════════════════════════════════════════

# Convert image format
dx convert ./image.png --to webp

# Convert with quality
dx convert ./photo.jpg --to webp --quality 80

# Resize image
dx convert ./large.jpg --to png --width 1920 --height 1080

# Batch convert
dx convert ./images/*.png --to jpg

# Convert video (requires FFmpeg)
dx convert ./video.mp4 --to webm

# Convert audio
dx convert ./audio.wav --to mp3 --bitrate 320k

# ═══════════════════════════════════════════════════════════════
# 🕷️  SCRAPE (Unlimited File Types)
# ═══════════════════════════════════════════════════════════════

# Scrape images from webpage
dx scrape "https://example.com/gallery" --type image

# Scrape with file pattern
dx scrape "https://archive.org/details/dataset" --pattern "*.csv"

# Recursive scraping
dx scrape "https://docs.example.com" --pattern "*.pdf" --depth 3

# Scrape article to markdown
dx scrape "https://en.wikipedia.org/wiki/Rust" --type article --to markdown

# Scrape any file type
dx scrape "https://example.com" --pattern "*.{blend,fbx,obj}"

# ═══════════════════════════════════════════════════════════════
# 📋 LIST & INFO
# ═══════════════════════════════════════════════════════════════

# List downloaded assets
dx list

# List by type
dx list --type video

# Show detailed info
dx list --detailed

# Show statistics
dx stats

# Show specific asset info
dx info ./media/images/sunset.jpg

# ═══════════════════════════════════════════════════════════════
# 🔌 PROVIDERS
# ═══════════════════════════════════════════════════════════════

# List all providers
dx providers

# List available (with API keys)
dx providers --available

# List by media type
dx providers --type audio

# Test provider connection
dx providers --test unsplash

# Show provider info
dx providers --info pexels

# ═══════════════════════════════════════════════════════════════
# ⚙️  CONFIG
# ═══════════════════════════════════════════════════════════════

# Show config
dx config --show

# Set value
dx config --set download.concurrent=10

# Reset defaults
dx config --reset

# ═══════════════════════════════════════════════════════════════
# 🚀 ADVANCED
# ═══════════════════════════════════════════════════════════════

# Interactive mode
dx interactive

# Pipeline mode
dx search "cat" --type image --json | dx download --stdin

# Watch folder for new files
dx watch ./incoming --convert webp --output ./processed

# Generate asset report
dx report --format html --output ./report.html
```

### Rust Library Usage

```rust
use dx_media::{DxMedia, MediaType, SearchQuery, ConversionOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize
    let dx = DxMedia::new()?;
    
    // ═══════════════════════════════════════════════════════════
    // Simple Search
    // ═══════════════════════════════════════════════════════════
    
    let results = dx.search("nature landscape", MediaType::Image).await?;
    
    println!("Found {} assets from {} providers", 
        results.total_assets,
        results.providers_searched.len()
    );
    
    // Download first result
    if let Some(asset) = results.all_assets().first() {
        let path = dx.download(asset).await?;
        println!("Downloaded: {:?}", path);
    }
    
    // ═══════════════════════════════════════════════════════════
    // Advanced Search with Filters
    // ═══════════════════════════════════════════════════════════
    
    let query = SearchQuery::new("sunset", MediaType::Photo)
        .with_providers(vec!["unsplash", "pexels"])
        .with_orientation(Orientation::Landscape)
        .with_color("orange")
        .with_min_width(1920)
        .with_per_page(50);
    
    let results = dx.search_with_query(query).await?;
    
    // ═══════════════════════════════════════════════════════════
    // Search Specific Provider
    // ═══════════════════════════════════════════════════════════
    
    let videos = dx.search_provider("pexels", "ocean waves", MediaType::Video).await?;
    
    // ═══════════════════════════════════════════════════════════
    // Batch Download
    // ═══════════════════════════════════════════════════════════
    
    let assets = results.all_assets();
    let paths = dx.download_batch(&assets[..10]).await?;
    
    // ═══════════════════════════════════════════════════════════
    // Convert Assets
    // ═══════════════════════════════════════════════════════════
    
    let options = ConversionOptions::new("webp")
        .with_quality(85)
        .with_max_width(1920);
    
    dx.convert("./image.png", options).await?;
    
    // ═══════════════════════════════════════════════════════════
    // Scrape Website
    // ═══════════════════════════════════════════════════════════
    
    let scraped = dx.scrape("https://example.com/gallery")
        .with_pattern("*.jpg")
        .with_depth(2)
        .execute()
        .await?;
    
    // ═══════════════════════════════════════════════════════════
    // Get Provider Information
    // ═══════════════════════════════════════════════════════════
    
    let providers = dx.providers_for(MediaType::Audio);
    for provider in providers {
        println!("{}: available={}", provider.name(), provider.is_available());
    }
    
    Ok(())
}
```

---

## 🗺️ Roadmap

### ✅ Phase 1: Foundation (Complete)
- [x] Core architecture with async runtime
- [x] 150+ media type definitions
- [x] Provider trait system
- [x] Basic CLI structure
- [x] Configuration management

### 🚧 Phase 2: Core Providers (In Progress)
- [x] Image providers (Unsplash, Pexels, Pixabay)
- [x] Text providers (Wikipedia, Gutenberg)
- [ ] Video providers (Pexels, Pixabay, Coverr)
- [ ] Audio providers (Freesound, Jamendo)
- [ ] Data providers (GitHub, Data.gov)

### 📋 Phase 3: Advanced Features
- [ ] Full scraper engine
- [ ] Format conversion (FFmpeg integration)
- [ ] Intelligent caching
- [ ] Progress tracking & resume
- [ ] Batch operations

### 🔮 Phase 4: Power Features
- [ ] 3D asset providers
- [ ] Document providers
- [ ] Game asset providers
- [ ] Interactive TUI mode
- [ ] Plugin system

### 🚀 Phase 5: Enterprise Features
- [ ] Cloud storage integration
- [ ] Team collaboration
- [ ] Asset versioning
- [ ] AI-powered search
- [ ] Web dashboard

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/dx-media.git

# Create feature branch
git checkout -b feature/awesome-feature

# Make changes and test
cargo test
cargo clippy
cargo fmt

# Submit PR
```

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- **Rust Community** — For the incredible ecosystem
- **Free API Providers** — Making this possible
- **Open Source Contributors** — Building the tools we use
- **You** — For using DX Media!

---

<div align="center">

```
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   ██████╗ ██╗  ██╗    ███╗   ███╗███████╗██████╗ ██╗ █████╗   ║
║   ██╔══██╗╚██╗██╔╝    ████╗ ████║██╔════╝██╔══██╗██║██╔══██╗  ║
║   ██║  ██║ ╚███╔╝     ██╔████╔██║█████╗  ██║  ██║██║███████║  ║
║   ██║  ██║ ██╔██╗     ██║╚██╔╝██║██╔══╝  ██║  ██║██║██╔══██║  ║
║   ██████╔╝██╔╝ ██╗    ██║ ╚═╝ ██║███████╗██████╔╝██║██║  ██║  ║
║   ╚═════╝ ╚═╝  ╚═╝    ╚═╝     ╚═╝╚══════╝╚═════╝ ╚═╝╚═╝  ╚═╝  ║
║                                                               ║
║           ONE COMMAND. ANY MEDIA. FROM ANYWHERE.              ║
║                                                               ║
║   ⭐ Star us on GitHub — it motivates us a lot! ⭐            ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

**Made with ❤️ and 🦀 by the DX Team**

[Website](https://dx-media.dev) •
[Documentation](https://docs.dx-media.dev) •
[Discord](https://discord.gg/dx-media) •
[Twitter](https://twitter.com/dx_media)

</div>