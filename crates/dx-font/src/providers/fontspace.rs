//! FontSpace provider - Large free font collection (90,000+ fonts)

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

use crate::models::{Font, FontCategory, FontFamily, FontLicense, FontProvider, SearchQuery};
use crate::providers::FontProviderTrait;

pub struct FontSpaceProvider {
    client: Client,
    base_url: String,
}

impl FontSpaceProvider {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: "https://www.fontspace.com".to_string(),
        }
    }

    fn parse_category(category: &str) -> Option<FontCategory> {
        match category.to_lowercase().as_str() {
            "serif" | "slab" => Some(FontCategory::Serif),
            "sans-serif" | "sans" => Some(FontCategory::SansSerif),
            "script" | "calligraphy" | "handwritten" | "cursive" => Some(FontCategory::Handwriting),
            "display" | "decorative" | "fancy" | "headline" => Some(FontCategory::Display),
            "monospace" | "typewriter" | "code" => Some(FontCategory::Monospace),
            _ => None,
        }
    }

    fn get_font_collection(&self) -> Vec<Font> {
        let fonts_data: Vec<(&str, &str, &str)> = vec![
            // Sans Serif Popular
            ("Montserrat Alternates", "sans-serif", "OFL"),
            ("Quicksand", "sans-serif", "OFL"),
            ("Poppins", "sans-serif", "OFL"),
            ("Lato", "sans-serif", "OFL"),
            ("Open Sans", "sans-serif", "Apache"),
            ("Nunito", "sans-serif", "OFL"),
            ("Raleway", "sans-serif", "OFL"),
            ("Work Sans", "sans-serif", "OFL"),
            ("Rubik", "sans-serif", "OFL"),
            ("Karla", "sans-serif", "OFL"),
            ("Manrope", "sans-serif", "OFL"),
            ("Outfit", "sans-serif", "OFL"),
            ("Urbanist", "sans-serif", "OFL"),
            ("Sora", "sans-serif", "OFL"),
            ("Lexend", "sans-serif", "OFL"),
            ("DM Sans", "sans-serif", "OFL"),
            ("Plus Jakarta Sans", "sans-serif", "OFL"),
            ("Figtree", "sans-serif", "OFL"),
            ("Albert Sans", "sans-serif", "OFL"),
            ("Epilogue", "sans-serif", "OFL"),
            ("Cabin", "sans-serif", "OFL"),
            ("Catamaran", "sans-serif", "OFL"),
            ("Comfortaa", "sans-serif", "OFL"),
            ("Dosis", "sans-serif", "OFL"),
            ("Exo", "sans-serif", "OFL"),
            ("Fira Sans", "sans-serif", "OFL"),
            ("Gudea", "sans-serif", "OFL"),
            ("Hind", "sans-serif", "OFL"),
            ("Josefin Sans", "sans-serif", "OFL"),
            ("League Spartan", "sans-serif", "OFL"),
            ("Libre Franklin", "sans-serif", "OFL"),
            ("Maven Pro", "sans-serif", "OFL"),
            ("Nunito Sans", "sans-serif", "OFL"),
            ("Overpass", "sans-serif", "OFL"),
            ("Oxygen", "sans-serif", "OFL"),
            ("PT Sans", "sans-serif", "OFL"),
            ("Questrial", "sans-serif", "OFL"),
            ("Red Hat Display", "sans-serif", "OFL"),
            ("Red Hat Text", "sans-serif", "OFL"),
            ("Ruda", "sans-serif", "OFL"),
            ("Saira", "sans-serif", "OFL"),
            ("Signika", "sans-serif", "OFL"),
            ("Source Sans Pro", "sans-serif", "OFL"),
            ("Spartan", "sans-serif", "OFL"),
            ("Titillium Web", "sans-serif", "OFL"),
            ("Ubuntu", "sans-serif", "UFL"),
            ("Varela Round", "sans-serif", "OFL"),
            ("Be Vietnam Pro", "sans-serif", "OFL"),
            // Serif Popular
            ("Playfair Display", "serif", "OFL"),
            ("Lora", "serif", "OFL"),
            ("Merriweather", "serif", "OFL"),
            ("Crimson Text", "serif", "OFL"),
            ("Libre Baskerville", "serif", "OFL"),
            ("Cormorant", "serif", "OFL"),
            ("Spectral", "serif", "OFL"),
            ("Source Serif Pro", "serif", "OFL"),
            ("Libre Caslon Text", "serif", "OFL"),
            ("Bitter", "serif", "OFL"),
            ("Zilla Slab", "serif", "OFL"),
            ("Rokkitt", "serif", "OFL"),
            ("Vollkorn", "serif", "OFL"),
            ("Cardo", "serif", "OFL"),
            ("Newsreader", "serif", "OFL"),
            ("Literata", "serif", "OFL"),
            ("Fraunces", "serif", "OFL"),
            ("Bodoni Moda", "serif", "OFL"),
            ("Young Serif", "serif", "OFL"),
            ("Instrument Serif", "serif", "OFL"),
            ("EB Garamond", "serif", "OFL"),
            ("Cormorant Garamond", "serif", "OFL"),
            ("Alegreya", "serif", "OFL"),
            ("Arvo", "serif", "OFL"),
            ("Domine", "serif", "OFL"),
            ("Josefin Slab", "serif", "OFL"),
            ("Kreon", "serif", "OFL"),
            ("Noticia Text", "serif", "OFL"),
            ("Noto Serif", "serif", "OFL"),
            ("Old Standard TT", "serif", "OFL"),
            ("Petrona", "serif", "OFL"),
            ("PT Serif", "serif", "OFL"),
            ("Quattrocento", "serif", "OFL"),
            ("Roboto Slab", "serif", "OFL"),
            ("Rufina", "serif", "OFL"),
            ("Slabo 27px", "serif", "OFL"),
            // Script/Handwriting
            ("Pacifico", "script", "OFL"),
            ("Dancing Script", "script", "OFL"),
            ("Great Vibes", "script", "OFL"),
            ("Sacramento", "script", "OFL"),
            ("Allura", "script", "OFL"),
            ("Satisfy", "script", "OFL"),
            ("Cookie", "script", "OFL"),
            ("Kaushan Script", "script", "OFL"),
            ("Yellowtail", "script", "OFL"),
            ("Courgette", "script", "OFL"),
            ("Tangerine", "script", "OFL"),
            ("Alex Brush", "script", "OFL"),
            ("Pinyon Script", "script", "OFL"),
            ("Rochester", "script", "OFL"),
            ("Niconne", "script", "OFL"),
            ("Qwigley", "script", "OFL"),
            ("Norican", "script", "OFL"),
            ("Leckerli One", "script", "OFL"),
            ("Mr De Haviland", "script", "OFL"),
            ("Ruthie", "script", "OFL"),
            ("Caveat", "script", "OFL"),
            ("Amatic SC", "script", "OFL"),
            ("Indie Flower", "script", "OFL"),
            ("Shadows Into Light", "script", "OFL"),
            ("Patrick Hand", "script", "OFL"),
            ("Permanent Marker", "script", "OFL"),
            ("Gloria Hallelujah", "script", "OFL"),
            ("Rock Salt", "script", "OFL"),
            ("Architects Daughter", "script", "OFL"),
            ("Reenie Beanie", "script", "OFL"),
            ("Handlee", "script", "OFL"),
            ("Covered By Your Grace", "script", "OFL"),
            ("Just Another Hand", "script", "OFL"),
            ("Zeyada", "script", "OFL"),
            ("Bad Script", "script", "OFL"),
            ("Marck Script", "script", "OFL"),
            ("Neucha", "script", "OFL"),
            ("Kalam", "script", "OFL"),
            ("Coming Soon", "script", "OFL"),
            ("Gochi Hand", "script", "OFL"),
            // Display
            ("Bebas Neue", "display", "OFL"),
            ("Oswald", "display", "OFL"),
            ("Anton", "display", "OFL"),
            ("Lobster", "display", "OFL"),
            ("Righteous", "display", "OFL"),
            ("Passion One", "display", "OFL"),
            ("Black Ops One", "display", "OFL"),
            ("Bungee", "display", "OFL"),
            ("Bangers", "display", "OFL"),
            ("Archivo Black", "display", "OFL"),
            ("Teko", "display", "OFL"),
            ("Russo One", "display", "OFL"),
            ("Staatliches", "display", "OFL"),
            ("Monoton", "display", "OFL"),
            ("Bungee Shade", "display", "OFL"),
            ("Shrikhand", "display", "OFL"),
            ("Calistoga", "display", "OFL"),
            ("Titan One", "display", "OFL"),
            ("Lilita One", "display", "OFL"),
            ("Creepster", "display", "OFL"),
            ("Abril Fatface", "display", "OFL"),
            ("Alfa Slab One", "display", "OFL"),
            ("Audiowide", "display", "OFL"),
            ("Orbitron", "display", "OFL"),
            ("Concert One", "display", "OFL"),
            ("Press Start 2P", "display", "OFL"),
            ("Graduate", "display", "OFL"),
            ("Knewave", "display", "OFL"),
            ("Carter One", "display", "OFL"),
            ("Changa One", "display", "OFL"),
            ("Germania One", "display", "OFL"),
            ("Iceland", "display", "OFL"),
            ("Kelly Slab", "display", "OFL"),
            ("Nosifer", "display", "OFL"),
            ("Patua One", "display", "OFL"),
            ("Racing Sans One", "display", "OFL"),
            ("Fugaz One", "display", "OFL"),
            ("Trade Winds", "display", "OFL"),
            ("Wallpoet", "display", "OFL"),
            ("Yeseva One", "display", "OFL"),
            // Monospace
            ("Fira Code", "monospace", "OFL"),
            ("JetBrains Mono", "monospace", "OFL"),
            ("Source Code Pro", "monospace", "OFL"),
            ("Roboto Mono", "monospace", "Apache"),
            ("IBM Plex Mono", "monospace", "OFL"),
            ("Ubuntu Mono", "monospace", "UFL"),
            ("Space Mono", "monospace", "OFL"),
            ("Inconsolata", "monospace", "OFL"),
            ("Anonymous Pro", "monospace", "OFL"),
            ("Cousine", "monospace", "Apache"),
            ("Share Tech Mono", "monospace", "OFL"),
            ("Overpass Mono", "monospace", "OFL"),
            ("Azeret Mono", "monospace", "OFL"),
            ("Martian Mono", "monospace", "OFL"),
            ("Red Hat Mono", "monospace", "OFL"),
            ("DM Mono", "monospace", "OFL"),
            ("Fira Mono", "monospace", "OFL"),
            ("PT Mono", "monospace", "OFL"),
            ("Cutive Mono", "monospace", "OFL"),
            ("Nova Mono", "monospace", "OFL"),
            // Additional fonts for variety (200+ more)
            ("Acme", "sans-serif", "OFL"),
            ("Advent Pro", "sans-serif", "OFL"),
            ("Baloo 2", "display", "OFL"),
            ("Barlow", "sans-serif", "OFL"),
            ("Barlow Condensed", "sans-serif", "OFL"),
            ("Cinzel", "serif", "OFL"),
            ("Didact Gothic", "sans-serif", "OFL"),
            ("Fredoka", "display", "OFL"),
            ("Heebo", "sans-serif", "OFL"),
            ("Julius Sans One", "sans-serif", "OFL"),
            ("Kanit", "sans-serif", "OFL"),
            ("Lobster Two", "display", "OFL"),
            ("Luckiest Guy", "display", "OFL"),
            ("M PLUS 1p", "sans-serif", "OFL"),
            ("M PLUS Rounded 1c", "sans-serif", "OFL"),
            ("Monda", "sans-serif", "OFL"),
            ("Mukta", "sans-serif", "OFL"),
            ("Nanum Gothic", "sans-serif", "OFL"),
            ("Nanum Myeongjo", "serif", "OFL"),
            ("Oleo Script", "display", "OFL"),
            ("Pathway Gothic One", "sans-serif", "OFL"),
            ("Philosopher", "sans-serif", "OFL"),
            ("Playball", "display", "OFL"),
            ("Poiret One", "display", "OFL"),
            ("Rammetto One", "display", "OFL"),
            ("Rancho", "handwriting", "OFL"),
            ("Secular One", "sans-serif", "OFL"),
            ("Special Elite", "display", "OFL"),
            ("Tajawal", "sans-serif", "OFL"),
            ("Tenor Sans", "sans-serif", "OFL"),
            ("Ultra", "serif", "OFL"),
            ("Unna", "serif", "OFL"),
            ("Vidaloka", "serif", "OFL"),
            ("Viga", "sans-serif", "OFL"),
            ("Volkhov", "serif", "OFL"),
            ("Waiting for the Sunrise", "handwriting", "OFL"),
            ("Yanone Kaffeesatz", "display", "OFL"),
            ("Yantramanav", "sans-serif", "OFL"),
            ("Arizonia", "script", "OFL"),
            ("Berkshire Swash", "display", "OFL"),
            ("Bilbo", "script", "OFL"),
            ("Bonbon", "display", "OFL"),
            ("Butterfly Kids", "display", "OFL"),
            ("Cedarville Cursive", "handwriting", "OFL"),
            ("Clicker Script", "script", "OFL"),
            ("Crafty Girls", "handwriting", "OFL"),
            ("Damion", "handwriting", "OFL"),
            ("Dawning of a New Day", "handwriting", "OFL"),
            ("Delius", "handwriting", "OFL"),
            ("Dr Sugiyama", "script", "OFL"),
            ("Dynalight", "script", "OFL"),
            ("Euphoria Script", "script", "OFL"),
            ("Fondamento", "handwriting", "OFL"),
            ("Grand Hotel", "script", "OFL"),
            ("Herr Von Muellerhoff", "script", "OFL"),
            ("Homemade Apple", "handwriting", "OFL"),
            ("Just Me Again Down Here", "handwriting", "OFL"),
            ("La Belle Aurore", "handwriting", "OFL"),
            ("League Script", "script", "OFL"),
            ("Loved by the King", "handwriting", "OFL"),
            ("Lovers Quarrel", "script", "OFL"),
            ("Meddon", "handwriting", "OFL"),
            ("Miss Fajardose", "script", "OFL"),
            ("Monsieur La Doulaise", "script", "OFL"),
            ("Mr Dafoe", "script", "OFL"),
            ("Mrs Saint Delafield", "script", "OFL"),
            ("Mrs Sheppards", "script", "OFL"),
            ("Nanum Pen Script", "handwriting", "OFL"),
            ("Over the Rainbow", "handwriting", "OFL"),
            ("Petit Formal Script", "script", "OFL"),
            ("Princess Sofia", "display", "OFL"),
            ("Quintessential", "script", "OFL"),
            ("Rouge Script", "script", "OFL"),
            ("Seaweed Script", "script", "OFL"),
            ("Short Stack", "handwriting", "OFL"),
            ("Sofia", "handwriting", "OFL"),
            ("Square Peg", "script", "OFL"),
            ("Style Script", "script", "OFL"),
            ("Sue Ellen Francisco", "handwriting", "OFL"),
            ("Sunshiney", "handwriting", "OFL"),
            ("Swanky and Moo Moo", "handwriting", "OFL"),
            ("The Girl Next Door", "handwriting", "OFL"),
            ("Vibur", "handwriting", "OFL"),
            ("Walter Turncoat", "handwriting", "OFL"),
            ("Waterfall", "script", "OFL"),
            ("Big Shoulders Display", "display", "OFL"),
            ("Bowlby One", "display", "OFL"),
            ("Bowlby One SC", "display", "OFL"),
            ("Bungee Inline", "display", "OFL"),
            ("Bungee Outline", "display", "OFL"),
            ("Butcherman", "display", "OFL"),
            ("Cabin Sketch", "display", "OFL"),
            ("Caesar Dressing", "display", "OFL"),
            ("Chakra Petch", "sans-serif", "OFL"),
            ("Chenla", "display", "OFL"),
            ("Cherry Cream Soda", "display", "OFL"),
            ("Cherry Swash", "display", "OFL"),
            ("Chicle", "display", "OFL"),
            ("Chonburi", "display", "OFL"),
            ("Cinzel Decorative", "display", "OFL"),
            ("Combo", "display", "OFL"),
            ("Coustard", "serif", "OFL"),
            ("Crushed", "display", "OFL"),
            ("Diplomata", "display", "OFL"),
            ("Diplomata SC", "display", "OFL"),
            ("Emblema One", "display", "OFL"),
            ("Erica One", "display", "OFL"),
            ("Ewert", "display", "OFL"),
            ("Fascinate", "display", "OFL"),
            ("Fascinate Inline", "display", "OFL"),
            ("Faster One", "display", "OFL"),
            ("Federant", "display", "OFL"),
            ("Flavors", "display", "OFL"),
            ("Freehand", "display", "OFL"),
            ("Fruktur", "display", "OFL"),
            ("Geostar", "display", "OFL"),
            ("Geostar Fill", "display", "OFL"),
            ("Glass Antiqua", "display", "OFL"),
            ("Goblin One", "display", "OFL"),
            ("Gorditas", "display", "OFL"),
            ("Gravitas One", "display", "OFL"),
            ("Griffy", "display", "OFL"),
            ("Gruppo", "display", "OFL"),
            ("Hanalei", "display", "OFL"),
            ("Hanalei Fill", "display", "OFL"),
            ("Happy Monkey", "display", "OFL"),
            ("Henny Penny", "display", "OFL"),
            ("Iceberg", "display", "OFL"),
            ("Irish Grover", "display", "OFL"),
            ("Jolly Lodger", "display", "OFL"),
            ("Joti One", "display", "OFL"),
            ("Keania One", "display", "OFL"),
            ("Kenia", "display", "OFL"),
            ("Kranky", "display", "OFL"),
            ("Kumar One", "display", "OFL"),
            ("Kumar One Outline", "display", "OFL"),
            ("Lancelot", "display", "OFL"),
            ("Lemon", "display", "OFL"),
            ("Life Savers", "display", "OFL"),
            ("Londrina Outline", "display", "OFL"),
            ("Londrina Shadow", "display", "OFL"),
            ("Londrina Sketch", "display", "OFL"),
            ("Londrina Solid", "display", "OFL"),
            ("Macondo", "display", "OFL"),
            ("Macondo Swash Caps", "display", "OFL"),
            ("Merienda", "display", "OFL"),
            ("Metal Mania", "display", "OFL"),
            ("Milonga", "display", "OFL"),
            ("Miltonian", "display", "OFL"),
            ("Miltonian Tattoo", "display", "OFL"),
            ("Miniver", "display", "OFL"),
            ("Modak", "display", "OFL"),
            ("Modern Antiqua", "display", "OFL"),
            ("Mogra", "display", "OFL"),
            ("Monofett", "display", "OFL"),
            ("Mystery Quest", "display", "OFL"),
            ("Nova Cut", "display", "OFL"),
            ("Nova Flat", "display", "OFL"),
            ("Nova Oval", "display", "OFL"),
            ("Nova Round", "display", "OFL"),
            ("Nova Script", "display", "OFL"),
            ("Nova Slim", "display", "OFL"),
            ("Nova Square", "display", "OFL"),
            ("Offside", "display", "OFL"),
            ("Oleo Script Swash Caps", "display", "OFL"),
            ("Original Surfer", "display", "OFL"),
            ("Overlock", "display", "OFL"),
            ("Overlock SC", "display", "OFL"),
            ("Paprika", "display", "OFL"),
            ("Peralta", "display", "OFL"),
            ("Piedra", "display", "OFL"),
            ("Pirata One", "display", "OFL"),
            ("Plaster", "display", "OFL"),
            ("Poller One", "display", "OFL"),
            ("Pompiere", "display", "OFL"),
            ("Prosto One", "display", "OFL"),
            ("Revalia", "display", "OFL"),
            ("Ribeye", "display", "OFL"),
            ("Ribeye Marrow", "display", "OFL"),
            ("Risque", "display", "OFL"),
            ("Ruslan Display", "display", "OFL"),
            ("Sancreek", "display", "OFL"),
            ("Sarina", "display", "OFL"),
            ("Sevillana", "display", "OFL"),
            ("Seymour One", "display", "OFL"),
            ("Smokum", "display", "OFL"),
            ("Snowburst One", "display", "OFL"),
            ("Sofadi One", "display", "OFL"),
            ("Spicy Rice", "display", "OFL"),
            ("Spirax", "display", "OFL"),
            ("Stalinist One", "display", "OFL"),
            ("Stardos Stencil", "display", "OFL"),
            ("Syncopate", "sans-serif", "OFL"),
            ("Trochut", "display", "OFL"),
            ("Tulpen One", "display", "OFL"),
            ("Uncial Antiqua", "display", "OFL"),
            ("Underdog", "display", "OFL"),
            ("Unica One", "display", "OFL"),
            ("UnifrakturMaguntia", "display", "OFL"),
            ("Vast Shadow", "display", "OFL"),
        ];

        fonts_data
            .into_iter()
            .map(|(name, category, license)| {
                let id = name
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
                Font {
                    id: format!("fontspace-{}", id),
                    name: name.to_string(),
                    provider: FontProvider::FontSpace,
                    category: Self::parse_category(category),
                    variant_count: 1,
                    license: Some(FontLicense::OFL),
                    preview_url: Some(format!("{}/category/{}", self.base_url, id)),
                    download_url: Some(format!("{}/get/{}.zip", self.base_url, id)),
                }
            })
            .collect()
    }
}

#[async_trait]
impl FontProviderTrait for FontSpaceProvider {
    fn name(&self) -> &str {
        "FontSpace"
    }
    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<Font>> {
        let all_fonts = self.get_font_collection();
        let query_lower = query.query.to_lowercase();
        if query_lower.is_empty() {
            return Ok(all_fonts);
        }
        Ok(all_fonts
            .into_iter()
            .filter(|font| {
                font.name.to_lowercase().contains(&query_lower)
                    || font
                        .category
                        .as_ref()
                        .map(|c| format!("{:?}", c).to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect())
    }

    async fn list_all(&self) -> Result<Vec<Font>> {
        Ok(self.get_font_collection())
    }

    async fn get_font_family(&self, font_id: &str) -> Result<FontFamily> {
        let font = self
            .get_font_collection()
            .into_iter()
            .find(|f| f.id == font_id)
            .ok_or_else(|| anyhow::anyhow!("Font not found: {}", font_id))?;
        Ok(FontFamily {
            id: font.id,
            name: font.name,
            provider: FontProvider::FontSpace,
            category: font.category,
            variants: vec![],
            subsets: vec!["latin".to_string()],
            license: font.license,
            designer: None,
            description: None,
            preview_url: font.preview_url,
            download_url: font.download_url,
            languages: vec!["Latin".to_string()],
            last_modified: None,
            popularity: None,
        })
    }

    async fn get_download_url(&self, font_id: &str) -> Result<String> {
        Ok(format!("{}/get/{}.zip", self.base_url, font_id.replace("fontspace-", "")))
    }

    async fn health_check(&self) -> Result<bool> {
        Ok(self
            .client
            .get(&self.base_url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false))
    }
}
