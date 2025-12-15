//! Configuration for dx-font

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default output directory for downloaded fonts
    pub output_dir: PathBuf,
    
    /// Preferred font formats in order of priority
    pub preferred_formats: Vec<String>,
    
    /// Maximum concurrent downloads
    pub max_concurrent_downloads: usize,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// User agent for HTTP requests
    pub user_agent: String,
    
    /// Cache directory for API responses
    pub cache_dir: PathBuf,
    
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./fonts"),
            preferred_formats: vec![
                "woff2".to_string(),
                "woff".to_string(),
                "ttf".to_string(),
                "otf".to_string(),
            ],
            max_concurrent_downloads: 5,
            timeout_seconds: 30,
            user_agent: format!("dx-font/{}", env!("CARGO_PKG_VERSION")),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("dx-font"),
            cache_ttl_seconds: 3600, // 1 hour
        }
    }
}

/// Font source URLs - all free, commercial-use fonts
pub mod sources {
    /// Tier 1: Primary APIs (No Keys Required)
    pub mod primary {
        pub const GOOGLE_FONTS: &str = "https://fonts.google.com";
        pub const GOOGLE_FONTS_API: &str = "https://www.googleapis.com/webfonts/v1/webfonts";
        pub const BUNNY_FONTS: &str = "https://fonts.bunny.net";
        pub const BUNNY_FONTS_API: &str = "https://fonts.bunny.net/list";
        pub const GOOGLE_WEBFONTS_HELPER: &str = "https://gwfh.mranftl.com/api/fonts";
        pub const FONTSOURCE_API: &str = "https://api.fontsource.org/v1/fonts";
        pub const FONT_LIBRARY: &str = "https://fontlibrary.org";
    }
    
    /// Tier 2: Major Free Font Sites
    pub mod major_sites {
        pub const FONT_SQUIRREL: &str = "https://www.fontsquirrel.com";
        pub const DAFONT: &str = "https://www.dafont.com";
        pub const FONTS_1001: &str = "https://www.1001fonts.com/free-fonts-for-commercial-use";
        pub const FONTSPACE: &str = "https://www.fontspace.com/category/open-source";
        pub const ABSTRACT_FONTS: &str = "https://www.abstractfonts.com";
        pub const URBAN_FONTS: &str = "https://www.urbanfonts.com/free-fonts.htm";
        pub const FONT_ZONE: &str = "https://fontzone.net";
        pub const FFONTS: &str = "https://www.ffonts.net";
        pub const FONT_MEME: &str = "https://fontmeme.com/fonts";
        pub const FONT_RIVER: &str = "https://www.fontriver.com";
    }
    
    /// Tier 3: Curated Foundries (High Quality)
    pub mod curated {
        pub const FONTSHARE: &str = "https://www.fontshare.com";
        pub const FONTSHARE_API: &str = "https://api.fontshare.com/v2/fonts";
        pub const VELVETYNE: &str = "https://velvetyne.fr";
        pub const OPEN_FOUNDRY: &str = "https://open-foundry.com";
        pub const LEAGUE_OF_MOVEABLE_TYPE: &str = "https://www.theleagueofmoveabletype.com";
        pub const UNCUT: &str = "https://uncut.wtf";
        pub const COLLLETTTIVO: &str = "https://www.collletttivo.it";
        pub const OMNIBUS_TYPE: &str = "https://www.omnibus-type.com";
        pub const FREE_FACES_GALLERY: &str = "https://www.freefaces.gallery";
        pub const USE_MODIFY: &str = "https://usemodify.com";
        pub const BEAUTIFUL_WEB_TYPE: &str = "https://beautifulwebtype.com";
        pub const FONTAIN: &str = "https://fontain.org";
        pub const GOOD_FONTS: &str = "https://goodfonts.io";
        pub const BEFONTS: &str = "https://befonts.com";
        pub const LOST_TYPE: &str = "https://www.losttype.com";
        pub const ATIPO_FOUNDRY: &str = "https://www.atipofoundry.com";
    }
    
    /// Tier 4: GitHub Repositories
    pub mod github {
        pub const GOOGLE_FONTS_REPO: &str = "https://github.com/google/fonts";
        pub const FONTSOURCE_REPO: &str = "https://github.com/fontsource/fontsource";
        pub const ADOBE_FONTS: &str = "https://github.com/adobe-fonts";
        pub const NOTO_FONTS: &str = "https://github.com/notofonts";
        pub const MOZILLA_FIRA: &str = "https://github.com/mozilla/Fira";
        pub const IBM_PLEX: &str = "https://github.com/IBM/plex";
        pub const INTER: &str = "https://github.com/rsms/inter";
        pub const JETBRAINS_MONO: &str = "https://github.com/JetBrains/JetBrainsMono";
        pub const CASCADIA_CODE: &str = "https://github.com/microsoft/cascadia-code";
        pub const FIRA_CODE: &str = "https://github.com/tonsky/FiraCode";
        pub const VICTOR_MONO: &str = "https://github.com/rubjo/victor-mono";
        pub const HACK: &str = "https://github.com/source-foundry/Hack";
        pub const IOSEVKA: &str = "https://github.com/be5invis/Iosevka";
        pub const RECURSIVE: &str = "https://github.com/arrowtype/recursive";
        pub const MANROPE: &str = "https://github.com/sharanda/manrope";
        pub const PUBLIC_SANS: &str = "https://github.com/uswds/public-sans";
        pub const WORK_SANS: &str = "https://github.com/weiweihuanghuang/Work-Sans";
        pub const OVERPASS: &str = "https://github.com/RedHatOfficial/Overpass";
        pub const LEXEND: &str = "https://github.com/googlefonts/lexend";
        pub const ATKINSON_HYPERLEGIBLE: &str = "https://github.com/googlefonts/atkinson-hyperlegible";
        pub const MONONOKI: &str = "https://github.com/madmalik/mononoki";
        pub const FANTASQUE_SANS: &str = "https://github.com/belluzj/fantasque-sans";
        pub const MONOID: &str = "https://github.com/larsenwork/monoid";
        pub const HASKLIG: &str = "https://github.com/i-tu/Hasklig";
        pub const LIBERATION_FONTS: &str = "https://github.com/liberationfonts/liberation-fonts";
    }
    
    /// Tier 5: International/Multi-Language
    pub mod international {
        pub const NOTO_FONTS: &str = "https://fonts.google.com/noto";
        pub const ARABIC_FONTS: &str = "https://arabicfonts.net";
        pub const CHINAZ_FONTS: &str = "https://font.chinaz.com";
        pub const FREE_JAPANESE_FONTS: &str = "https://freejapanesefont.com";
        pub const NOONNU: &str = "https://noonnu.cc";
        pub const HINDI_FONTS: &str = "https://hindityping.com/fonts";
        pub const THAI_FONTS: &str = "https://f0nt.com";
        pub const FONTER_RU: &str = "https://fonter.ru";
        pub const FONTS_IR: &str = "https://fonts.ir";
        pub const TAMIL_FONTS: &str = "https://tamilfonts.net";
        pub const BENGALI_FONTS: &str = "https://banglafonts.net";
        pub const SMC_MALAYALAM: &str = "https://smc.org.in/fonts";
    }
    
    /// CDN Direct Access URLs
    pub mod cdn {
        pub const JSDELIVR_FONTSOURCE: &str = "https://cdn.jsdelivr.net/npm/@fontsource";
        pub const UNPKG_FONTSOURCE: &str = "https://unpkg.com/@fontsource";
        pub const BUNNY_FONTS_CDN: &str = "https://fonts.bunny.net/css";
        pub const GOOGLE_FONTS_CDN: &str = "https://fonts.googleapis.com/css2";
        pub const GITHUB_RAW: &str = "https://raw.githubusercontent.com/google/fonts/main/ofl";
    }
}
