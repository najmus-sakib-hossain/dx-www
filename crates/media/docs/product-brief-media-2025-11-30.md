# Product Brief: DX Media

**Date:** 2025-11-30  
**Author:** BMad  
**Context:** Startup Seed (Open-Source + Future Monetization)

---

## Executive Summary

**DX Media** is a universal digital asset acquisition engine built in Rust that eliminates the fragmented, time-consuming process of finding and downloading free media assets. By abstracting 50+ free API providers behind a single CLI/library interface, developers can search, download, and organize images, videos, audio, documents, datasets, and 3D assets with one command.

**Tagline:** *"One Command. Any Media. From Anywhere."*

**Business Model Path:** Open-source core (MIT) → Community adoption → Premium features/cloud services

---

## Core Vision

### Problem Statement

Developers waste significant time acquiring digital assets for their projects:

1. **Fragmentation** — 50+ free asset sources exist, each with different APIs, auth methods, and rate limits
2. **Friction** — Creating accounts, obtaining API keys, learning each API's quirks
3. **Manual Labor** — Downloading, renaming, organizing files into project structures
4. **Repetition** — Same workflow repeated for every project, every asset type
5. **Context Switching** — Leaving IDE to browser, breaking flow state

### Problem Impact

| Metric | Current State |
|--------|---------------|
| Time per asset search | 10-30 minutes |
| Browser tabs opened | 5-10 per search |
| Accounts created | 10+ across providers |
| Developer frustration | High (breaks flow) |

### Why Existing Solutions Fall Short

| Solution | Gap |
|----------|-----|
| Browser bookmarks | Manual, no automation, no integration |
| Individual API wrappers | One provider at a time, no unified search |
| Paid stock services | $29-299/month, not free content focused |
| Download managers | No search, no API integration |

### Proposed Solution

A **dual-mode tool** (CLI + Rust library) that:
- Provides unified search across 50+ free API providers
- Handles authentication, rate limiting, and retries automatically
- Downloads assets with intelligent naming and organization
- Preserves license/attribution metadata
- Integrates into CI/CD pipelines and development workflows

### Key Differentiators

1. **Free-First Philosophy** — Curated list of legitimately free APIs (no piracy)
2. **Rust Performance** — Async Tokio runtime for blazing-fast parallel downloads
3. **Developer Experience** — Beautiful CLI with progress bars, colors, interactive mode
4. **Library Mode** — Use as a crate in Rust projects for programmatic access
5. **Ethical by Default** — Attribution tracking, license metadata preservation
6. **Extensible** — Plugin system for custom providers

---

## Target Users

### Primary Users

**Individual Developers & Hobbyists**
- Building side projects, portfolios, learning projects
- Need quick access to placeholder images, test data, sample audio
- Value: Speed, simplicity, free access

**Indie Game Developers**
- Prototyping games, game jams
- Need sprites, sound effects, 3D models, textures
- Value: Variety of game assets, CC0/public domain licenses

### Secondary Users

**Startups & Small Teams**
- Building MVPs, prototypes, marketing materials
- Need stock photos, videos, datasets for demos
- Value: Cost savings, CI/CD integration

**Content Creators & Educators**
- Creating tutorials, documentation, courses
- Need diagrams, screenshots, B-roll, music
- Value: Royalty-free content, attribution help

**AI/ML Practitioners**
- Need training datasets, labeled images
- Value: Dataset providers (Kaggle, Data.gov), batch downloads

---

## Success Metrics

### Adoption Metrics (Open Source)
- GitHub stars: 1K → 5K → 10K progression
- Monthly npm/crates.io downloads
- Community contributors (PRs merged)
- Discord/community members

### Usage Metrics
- Daily active CLI users
- Assets downloaded per month
- Providers actively used
- Library integrations (dependents)

### Business Objectives
- Establish category leadership in "developer asset tools"
- Build email list for future premium announcements
- Create foundation for DX Suite ecosystem (dx-icons, dx-fonts)

---

## MVP Scope

### Core Features (MVP v0.1)

| Feature | Priority | Description |
|---------|----------|-------------|
| **Image Search** | P0 | Search Unsplash, Pexels, Pixabay simultaneously |
| **Image Download** | P0 | Async download with progress, auto-naming |
| **Provider Abstraction** | P0 | Unified `Provider` trait for all sources |
| **CLI Interface** | P0 | `dx search`, `dx download` commands |
| **Configuration** | P0 | `.env` file for API keys |
| **Rate Limiting** | P0 | Respect provider limits automatically |
| **Error Handling** | P0 | Graceful degradation, retry logic |

### MVP Providers (Images Only)

| Provider | API Key | Rate Limit | Priority |
|----------|---------|------------|----------|
| **Unsplash** | Required | 50/hour | P0 |
| **Pexels** | Required | 200/hour | P0 |
| **Pixabay** | Required | Unlimited | P0 |
| **Lorem Picsum** | None | Unlimited | P1 |
| **Placeholder.com** | None | Unlimited | P1 |

### Out of Scope for MVP

- Video/audio/3D providers (Phase 2)
- Format conversion (Phase 3)
- Scraper mode (Phase 3)
- Interactive TUI mode (Phase 4)
- Plugin system (Phase 4)
- Cloud storage integration (Phase 5)

### MVP Success Criteria

1. ✅ `dx search "sunset" --type image` returns results from 3 providers
2. ✅ `dx search "cat" --download --count 5` saves 5 images to `./media/images/`
3. ✅ Rate limits respected (no 429 errors in normal use)
4. ✅ Works on Windows, macOS, Linux
5. ✅ < 5 second response time for searches
6. ✅ Library mode: `dx_media::search("query").await` works

---

## Technical Preferences

### Stack (Confirmed from README)

| Layer | Technology |
|-------|------------|
| Language | Rust 2024 Edition |
| Async Runtime | Tokio |
| HTTP Client | reqwest (rustls-tls) |
| CLI Framework | clap (derive) |
| Serialization | serde + serde_json |
| Error Handling | thiserror + anyhow |
| Logging | tracing |
| Progress | indicatif |
| Config | dotenvy + config |

### Architecture Principles

1. **Async-First** — All I/O operations async
2. **Provider Trait** — Uniform interface for all providers
3. **Graceful Degradation** — Failed providers don't break search
4. **Parallel Downloads** — Configurable concurrency
5. **Offline-Friendly** — Cache API responses

---

## Risks and Assumptions

### Assumptions

1. Free API providers will remain free (tier limits may change)
2. Developers prefer CLI tools for asset management
3. Rust ecosystem continues growing (crates.io adoption)

### Risks

| Risk | Mitigation |
|------|------------|
| API provider shutdowns | Support 50+ providers, graceful fallback |
| Rate limit changes | Per-provider configurable limits, user overrides |
| License compliance issues | Clear attribution tracking, license metadata |
| Rust adoption barrier | Provide pre-built binaries, consider future Node wrapper |

---

## Future Vision

### Phase 2: Multi-Media
- Video providers (Pexels, Pixabay, Coverr)
- Audio providers (Freesound, Jamendo)
- Data providers (GitHub, Data.gov, Kaggle)

### Phase 3: Power Features
- Scraper engine for any website
- Format conversion (FFmpeg integration)
- Intelligent caching

### Phase 4: Developer Experience
- Interactive TUI mode (ratatui)
- Plugin system for custom providers
- 3D/game asset providers

### Phase 5: Monetization
- DX Media Cloud (hosted search, no API key management)
- Team features (shared asset libraries)
- Enterprise (SSO, audit logs, compliance)

---

## Supporting Materials

- **README.md** — Comprehensive product vision and API provider research
- **Provider Tables** — 50+ free APIs catalogued with rate limits, auth requirements

---

_This Product Brief captures the vision for DX Media as a startup-seed open-source project._

_Next: PRD workflow will transform this brief into detailed functional requirements._
