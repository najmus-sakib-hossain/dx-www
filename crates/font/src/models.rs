//! Core data models for dx-font

use serde::{Deserialize, Serialize};

/// Represents a font provider/source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FontProvider {
    // Tier 1: Primary APIs
    GoogleFonts,
    BunnyFonts,
    Fontsource,
    GoogleWebfontsHelper,
    FontLibrary,

    // Tier 2: Major Free Sites
    FontSquirrel,
    DaFont,
    Fonts1001,
    FontSpace,
    AbstractFonts,
    UrbanFonts,
    FontZone,
    FFonts,
    FontMeme,
    FontRiver,

    // Tier 3: Curated Foundries
    FontShare,
    Velvetyne,
    OpenFoundry,
    LeagueOfMoveableType,
    Uncut,
    Collletttivo,
    OmnibusType,
    FreeFacesGallery,
    UseModify,
    BeautifulWebType,
    Fontain,
    GoodFonts,
    Befonts,
    LostType,
    AtipoFoundry,

    // Tier 4: GitHub Repositories
    GitHub,

    // Tier 5: International
    NotoFonts,
    ArabicFonts,
    ChinazFonts,
    FreeJapaneseFonts,
    Noonnu,
    HindiFonts,
    ThaiFonts,
    FonterRu,
    FontsIr,
    TamilFonts,
    BengaliFonts,
    SMCMalayalam,

    // Custom/Other
    Custom(String),
}

impl FontProvider {
    pub fn name(&self) -> &str {
        match self {
            FontProvider::GoogleFonts => "Google Fonts",
            FontProvider::BunnyFonts => "Bunny Fonts",
            FontProvider::Fontsource => "Fontsource",
            FontProvider::GoogleWebfontsHelper => "Google Webfonts Helper",
            FontProvider::FontLibrary => "Font Library",
            FontProvider::FontSquirrel => "Font Squirrel",
            FontProvider::DaFont => "DaFont",
            FontProvider::Fonts1001 => "1001 Fonts",
            FontProvider::FontSpace => "FontSpace",
            FontProvider::AbstractFonts => "Abstract Fonts",
            FontProvider::UrbanFonts => "Urban Fonts",
            FontProvider::FontZone => "Font Zone",
            FontProvider::FFonts => "FFonts",
            FontProvider::FontMeme => "Font Meme",
            FontProvider::FontRiver => "Font River",
            FontProvider::FontShare => "FontShare",
            FontProvider::Velvetyne => "Velvetyne",
            FontProvider::OpenFoundry => "Open Foundry",
            FontProvider::LeagueOfMoveableType => "The League of Moveable Type",
            FontProvider::Uncut => "Uncut.wtf",
            FontProvider::Collletttivo => "Collletttivo",
            FontProvider::OmnibusType => "OMNIBUS-TYPE",
            FontProvider::FreeFacesGallery => "Free Faces Gallery",
            FontProvider::UseModify => "Use & Modify",
            FontProvider::BeautifulWebType => "Beautiful Web Type",
            FontProvider::Fontain => "Fontain",
            FontProvider::GoodFonts => "Good Fonts",
            FontProvider::Befonts => "Befonts",
            FontProvider::LostType => "Lost Type Co-op",
            FontProvider::AtipoFoundry => "Atipo Foundry",
            FontProvider::GitHub => "GitHub",
            FontProvider::NotoFonts => "Noto Fonts",
            FontProvider::ArabicFonts => "Arabic Fonts",
            FontProvider::ChinazFonts => "Chinaz Fonts",
            FontProvider::FreeJapaneseFonts => "Free Japanese Fonts",
            FontProvider::Noonnu => "Noonnu (Korean)",
            FontProvider::HindiFonts => "Hindi Fonts",
            FontProvider::ThaiFonts => "Thai Fonts",
            FontProvider::FonterRu => "Fonter.ru",
            FontProvider::FontsIr => "Fonts.ir",
            FontProvider::TamilFonts => "Tamil Fonts",
            FontProvider::BengaliFonts => "Bengali Fonts",
            FontProvider::SMCMalayalam => "SMC Malayalam",
            FontProvider::Custom(name) => name,
        }
    }

    pub fn base_url(&self) -> &str {
        match self {
            FontProvider::GoogleFonts => "https://fonts.google.com",
            FontProvider::BunnyFonts => "https://fonts.bunny.net",
            FontProvider::Fontsource => "https://api.fontsource.org/v1/fonts",
            FontProvider::GoogleWebfontsHelper => "https://gwfh.mranftl.com/api/fonts",
            FontProvider::FontLibrary => "https://fontlibrary.org",
            FontProvider::FontSquirrel => "https://www.fontsquirrel.com",
            FontProvider::DaFont => "https://www.dafont.com",
            FontProvider::Fonts1001 => "https://www.1001fonts.com",
            FontProvider::FontSpace => "https://www.fontspace.com",
            FontProvider::AbstractFonts => "https://www.abstractfonts.com",
            FontProvider::UrbanFonts => "https://www.urbanfonts.com",
            FontProvider::FontZone => "https://fontzone.net",
            FontProvider::FFonts => "https://www.ffonts.net",
            FontProvider::FontMeme => "https://fontmeme.com",
            FontProvider::FontRiver => "https://www.fontriver.com",
            FontProvider::FontShare => "https://www.fontshare.com",
            FontProvider::Velvetyne => "https://velvetyne.fr",
            FontProvider::OpenFoundry => "https://open-foundry.com",
            FontProvider::LeagueOfMoveableType => "https://www.theleagueofmoveabletype.com",
            FontProvider::Uncut => "https://uncut.wtf",
            FontProvider::Collletttivo => "https://www.collletttivo.it",
            FontProvider::OmnibusType => "https://www.omnibus-type.com",
            FontProvider::FreeFacesGallery => "https://www.freefaces.gallery",
            FontProvider::UseModify => "https://usemodify.com",
            FontProvider::BeautifulWebType => "https://beautifulwebtype.com",
            FontProvider::Fontain => "https://fontain.org",
            FontProvider::GoodFonts => "https://goodfonts.io",
            FontProvider::Befonts => "https://befonts.com",
            FontProvider::LostType => "https://www.losttype.com",
            FontProvider::AtipoFoundry => "https://www.atipofoundry.com",
            FontProvider::GitHub => "https://github.com",
            FontProvider::NotoFonts => "https://fonts.google.com/noto",
            FontProvider::ArabicFonts => "https://arabicfonts.net",
            FontProvider::ChinazFonts => "https://font.chinaz.com",
            FontProvider::FreeJapaneseFonts => "https://freejapanesefont.com",
            FontProvider::Noonnu => "https://noonnu.cc",
            FontProvider::HindiFonts => "https://hindityping.com/fonts",
            FontProvider::ThaiFonts => "https://f0nt.com",
            FontProvider::FonterRu => "https://fonter.ru",
            FontProvider::FontsIr => "https://fonts.ir",
            FontProvider::TamilFonts => "https://tamilfonts.net",
            FontProvider::BengaliFonts => "https://banglafonts.net",
            FontProvider::SMCMalayalam => "https://smc.org.in/fonts",
            FontProvider::Custom(url) => url,
        }
    }
}

/// Font weight enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    Regular,    // 400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}

impl FontWeight {
    pub fn from_numeric(weight: u16) -> Self {
        match weight {
            0..=150 => FontWeight::Thin,
            151..=250 => FontWeight::ExtraLight,
            251..=350 => FontWeight::Light,
            351..=450 => FontWeight::Regular,
            451..=550 => FontWeight::Medium,
            551..=650 => FontWeight::SemiBold,
            651..=750 => FontWeight::Bold,
            751..=850 => FontWeight::ExtraBold,
            _ => FontWeight::Black,
        }
    }

    pub fn to_numeric(&self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

/// Font style (normal/italic)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal,
    Italic,
}

/// Font category/classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FontCategory {
    Serif,
    SansSerif,
    Display,
    Handwriting,
    Monospace,
}

/// License type for fonts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FontLicense {
    OFL,     // SIL Open Font License
    Apache2, // Apache License 2.0
    MIT,
    GPL,
    PublicDomain,
    FreeCommercial,
    Custom(String),
}

/// A single font variant (e.g., Regular, Bold Italic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontVariant {
    pub weight: FontWeight,
    pub style: FontStyle,
    pub file_url: Option<String>,
    pub file_format: String, // ttf, otf, woff, woff2
}

/// Represents a font family with all its variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamily {
    pub id: String,
    pub name: String,
    pub provider: FontProvider,
    pub category: Option<FontCategory>,
    pub variants: Vec<FontVariant>,
    pub license: Option<FontLicense>,
    pub designer: Option<String>,
    pub description: Option<String>,
    pub preview_url: Option<String>,
    pub download_url: Option<String>,
    pub languages: Vec<String>,
    pub subsets: Vec<String>,
    pub popularity: Option<u32>,
    pub last_modified: Option<String>,
}

/// A simplified font representation for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub id: String,
    pub name: String,
    pub provider: FontProvider,
    pub category: Option<FontCategory>,
    pub variant_count: usize,
    pub license: Option<FontLicense>,
    pub preview_url: Option<String>,
    pub download_url: Option<String>,
}

impl From<FontFamily> for Font {
    fn from(family: FontFamily) -> Self {
        Font {
            id: family.id,
            name: family.name,
            provider: family.provider,
            category: family.category,
            variant_count: family.variants.len(),
            license: family.license,
            preview_url: family.preview_url,
            download_url: family.download_url,
        }
    }
}

/// Search query parameters
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub query: String,
    pub providers: Option<Vec<FontProvider>>,
    pub category: Option<FontCategory>,
    pub license: Option<FontLicense>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Search results with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub fonts: Vec<Font>,
    pub total: usize,
    pub query: String,
    pub providers_searched: Vec<String>,
}

/// Download options
#[derive(Debug, Clone)]
pub struct DownloadOptions {
    pub output_dir: std::path::PathBuf,
    pub formats: Vec<String>, // ttf, otf, woff, woff2
    pub weights: Option<Vec<FontWeight>>,
    pub styles: Option<Vec<FontStyle>>,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            output_dir: std::path::PathBuf::from("./fonts"),
            formats: vec!["ttf".to_string(), "woff2".to_string()],
            weights: None,
            styles: None,
        }
    }
}
