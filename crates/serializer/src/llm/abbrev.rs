//! Key abbreviation dictionary for LLM and Human format conversion
//!
//! This module provides bidirectional key abbreviation mappings for converting
//! between token-efficient LLM format and human-readable format.

use std::collections::HashMap;

/// Bidirectional key abbreviation dictionary
///
/// Provides mappings between abbreviated keys (for LLM format) and
/// full key names (for Human format), with context-aware expansion
/// for ambiguous abbreviations.
#[derive(Debug, Clone)]
pub struct AbbrevDict {
    /// Short → Full (for expansion)
    global: HashMap<&'static str, &'static str>,
    /// Context-specific expansions: (abbrev, context) → full
    contextual: HashMap<(&'static str, &'static str), &'static str>,
    /// Full → Short (for compression)
    reverse: HashMap<&'static str, &'static str>,
}

impl AbbrevDict {
    /// Create dictionary with all standard mappings
    pub fn new() -> Self {
        let mut global = HashMap::new();
        let mut reverse = HashMap::new();
        let mut contextual = HashMap::new();

        // Helper to add bidirectional mapping
        let mut add = |abbrev: &'static str, full: &'static str| {
            global.insert(abbrev, full);
            reverse.insert(full, abbrev);
        };

        // === Identity ===
        add("id", "id");
        add("nm", "name");
        add("tt", "title");
        add("ds", "description");
        add("lb", "label");
        add("al", "alias");

        // === State ===
        add("st", "status");
        add("ac", "active");
        add("en", "enabled");
        add("vs", "visible");
        add("lk", "locked");
        add("ar", "archived");
        add("dl", "deleted");
        add("cp", "completed");
        add("pn", "pending");

        // === Timestamps ===
        add("cr", "created");
        add("up", "updated");
        add("dt", "date");
        add("tm", "time");
        add("ts", "timestamp");
        add("ex", "expires");
        add("du", "duration");
        add("yr", "year");
        add("mo", "month");
        add("dy", "day");

        // === Metrics ===
        add("ct", "count");
        add("tl", "total");
        add("am", "amount");
        add("pr", "price");
        add("qt", "quantity");
        add("km", "kilometers");
        add("mi", "miles");
        add("el", "elevation");
        add("rt", "rating");
        add("sc", "score");
        add("rk", "rank");
        add("pct", "percent");
        add("avg", "average");
        add("min", "minimum");
        add("max", "maximum");

        // === Dimensions ===
        add("wd", "width");
        add("ht", "height");
        add("sz", "size");
        add("len", "length");
        add("dp", "depth");
        add("wt", "weight");

        // === Web ===
        add("ur", "url");
        add("pt", "path");
        add("lnk", "link");
        add("src", "source");
        add("dst", "destination");
        add("ref", "reference");
        add("dom", "domain");
        add("api", "api");

        // === Contact ===
        add("em", "email");
        add("ph", "phone");
        add("ad", "address");
        add("fn", "first_name");
        add("lnm", "last_name");
        add("cmp", "company");

        // === Location ===
        add("cy", "city");
        add("co", "country");
        add("rg", "region");
        add("zp", "zipcode");
        add("la", "latitude");
        add("lo", "longitude");
        add("loc", "location");
        add("geo", "geo");

        // === Visual ===
        add("cl", "color");
        add("bg", "background");
        add("fg", "foreground");
        add("im", "image");
        add("ic", "icon");
        add("th", "thumbnail");

        // === Relations ===
        add("pa", "parent");
        add("ch", "children");
        add("us", "user");
        add("ow", "owner");
        add("au", "author");
        add("ed", "editor");
        add("rv", "reviewer");
        add("asg", "assignee");
        add("mb", "member");
        add("gp", "group");
        add("tea", "team");
        add("org", "organization");

        // === Classification ===
        add("ca", "category");
        add("tg", "tags");
        add("tp", "type");
        add("vl", "value");
        add("ky", "key");
        add("md", "mode");
        add("lv", "level");
        add("pri", "priority");
        add("vr", "version");

        // === Project/Workspace ===
        add("ws", "workspace");
        add("repo", "repository");
        add("cont", "container");
        add("ci", "ci_cd");
        add("eds", "editors");

        // === Commerce ===
        add("sk", "sku");
        add("cu", "customer");
        add("sh", "shipping");
        add("pd", "paid");
        add("ord", "order");
        add("inv", "invoice");
        add("prd", "product");
        add("dsc", "discount");
        add("tx", "tax");

        // === Content ===
        add("txt", "text");
        add("msg", "message");
        add("cmt", "comment");
        add("nt", "note");
        add("sum", "summary");
        add("cnt", "content");
        add("bd", "body");
        add("hd", "header");
        add("ft", "footer");

        // Drop the `add` closure to release the mutable borrow
        drop(add);

        // === Short aliases (expand only, don't affect compression) ===
        // "v" expands to "version" but "version" compresses to "vr"
        global.insert("v", "version");

        // === Context-aware expansions for ambiguous single-letter keys ===
        // 's' expansions
        contextual.insert(("s", "hikes"), "sunny");
        contextual.insert(("s", "weather"), "sunny");
        contextual.insert(("s", "orders"), "status");
        contextual.insert(("s", "tasks"), "status");
        contextual.insert(("s", "config"), "season");
        contextual.insert(("s", "default"), "status");

        // 'w' expansions
        contextual.insert(("w", "hikes"), "with");
        contextual.insert(("w", "images"), "width");
        contextual.insert(("w", "products"), "weight");
        contextual.insert(("w", "default"), "width");

        // 't' expansions
        contextual.insert(("t", "config"), "task");
        contextual.insert(("t", "products"), "type");
        contextual.insert(("t", "events"), "time");
        contextual.insert(("t", "default"), "type");

        // 'l' expansions
        contextual.insert(("l", "geo"), "location");
        contextual.insert(("l", "maps"), "location");
        contextual.insert(("l", "text"), "length");
        contextual.insert(("l", "default"), "location");

        // 'n' expansions
        contextual.insert(("n", "users"), "name");
        contextual.insert(("n", "items"), "name");
        contextual.insert(("n", "math"), "number");
        contextual.insert(("n", "default"), "name");

        // 'd' expansions
        contextual.insert(("d", "calendar"), "date");
        contextual.insert(("d", "events"), "date");
        contextual.insert(("d", "items"), "description");
        contextual.insert(("d", "default"), "date");

        // 'c' expansions
        contextual.insert(("c", "metrics"), "count");
        contextual.insert(("c", "items"), "category");
        contextual.insert(("c", "visual"), "color");
        contextual.insert(("c", "default"), "count");

        // 'v' expansions
        contextual.insert(("v", "data"), "value");
        contextual.insert(("v", "software"), "version");
        contextual.insert(("v", "default"), "version"); // V2: default to version

        // 'p' expansions
        contextual.insert(("p", "commerce"), "price");
        contextual.insert(("p", "tasks"), "priority");
        contextual.insert(("p", "files"), "path");
        contextual.insert(("p", "default"), "price");

        Self {
            global,
            contextual,
            reverse,
        }
    }


    /// Expand abbreviated key to full name
    ///
    /// Uses context-aware expansion for ambiguous single-letter keys.
    /// Falls back to global dictionary, then returns original if not found.
    pub fn expand(&self, abbrev: &str, context: &str) -> String {
        // First try context-specific expansion
        if let Some(&full) = self.contextual.get(&(abbrev, context)) {
            return full.to_string();
        }

        // Try default context for single-letter keys
        if abbrev.len() == 1 {
            if let Some(&full) = self.contextual.get(&(abbrev, "default")) {
                return full.to_string();
            }
        }

        // Fall back to global dictionary
        if let Some(&full) = self.global.get(abbrev) {
            return full.to_string();
        }

        // Return original if not found
        abbrev.to_string()
    }

    /// Compress full key to abbreviation
    ///
    /// Uses the shortest unambiguous abbreviation from the dictionary.
    /// Returns original if not found.
    pub fn compress(&self, full: &str) -> String {
        if let Some(&abbrev) = self.reverse.get(full) {
            return abbrev.to_string();
        }

        // Return original if not found
        full.to_string()
    }

    /// Check if an abbreviation exists in the dictionary
    pub fn has_abbrev(&self, abbrev: &str) -> bool {
        self.global.contains_key(abbrev)
    }

    /// Check if a full key exists in the dictionary
    pub fn has_full(&self, full: &str) -> bool {
        self.reverse.contains_key(full)
    }

    /// Get all global abbreviation mappings
    pub fn global_mappings(&self) -> &HashMap<&'static str, &'static str> {
        &self.global
    }

    /// Get the number of global mappings
    pub fn len(&self) -> usize {
        self.global.len()
    }

    /// Check if the dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.global.is_empty()
    }
}

impl Default for AbbrevDict {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbrev_dict_has_50_plus_mappings() {
        let dict = AbbrevDict::new();
        assert!(dict.len() >= 50, "Dictionary should have at least 50 mappings, has {}", dict.len());
    }

    #[test]
    fn test_expand_basic() {
        let dict = AbbrevDict::new();
        
        assert_eq!(dict.expand("nm", ""), "name");
        assert_eq!(dict.expand("tt", ""), "title");
        assert_eq!(dict.expand("ds", ""), "description");
        assert_eq!(dict.expand("st", ""), "status");
        assert_eq!(dict.expand("cr", ""), "created");
        assert_eq!(dict.expand("up", ""), "updated");
        assert_eq!(dict.expand("pr", ""), "price");
        assert_eq!(dict.expand("qt", ""), "quantity");
        assert_eq!(dict.expand("em", ""), "email");
        assert_eq!(dict.expand("ur", ""), "url");
    }

    #[test]
    fn test_expand_context_aware() {
        let dict = AbbrevDict::new();
        
        // 's' in different contexts
        assert_eq!(dict.expand("s", "hikes"), "sunny");
        assert_eq!(dict.expand("s", "orders"), "status");
        assert_eq!(dict.expand("s", "config"), "season");
        
        // 'w' in different contexts
        assert_eq!(dict.expand("w", "hikes"), "with");
        assert_eq!(dict.expand("w", "images"), "width");
        assert_eq!(dict.expand("w", "products"), "weight");
        
        // 't' in different contexts
        assert_eq!(dict.expand("t", "config"), "task");
        assert_eq!(dict.expand("t", "products"), "type");
        assert_eq!(dict.expand("t", "events"), "time");
    }

    #[test]
    fn test_compress_basic() {
        let dict = AbbrevDict::new();
        
        assert_eq!(dict.compress("name"), "nm");
        assert_eq!(dict.compress("title"), "tt");
        assert_eq!(dict.compress("description"), "ds");
        assert_eq!(dict.compress("status"), "st");
        assert_eq!(dict.compress("created"), "cr");
        assert_eq!(dict.compress("updated"), "up");
        assert_eq!(dict.compress("email"), "em");
        assert_eq!(dict.compress("url"), "ur");
    }

    #[test]
    fn test_unknown_key_passthrough() {
        let dict = AbbrevDict::new();
        
        // Unknown abbreviations pass through unchanged
        assert_eq!(dict.expand("xyz", ""), "xyz");
        assert_eq!(dict.expand("unknown_key", ""), "unknown_key");
        
        // Unknown full keys pass through unchanged
        assert_eq!(dict.compress("xyz"), "xyz");
        assert_eq!(dict.compress("unknown_key"), "unknown_key");
    }

    #[test]
    fn test_round_trip_global() {
        let dict = AbbrevDict::new();
        
        // For all global mappings, compress then expand should return original
        for (&abbrev, &full) in dict.global_mappings() {
            let compressed = dict.compress(full);
            let expanded = dict.expand(&compressed, "");
            assert_eq!(expanded, full, "Round-trip failed for {} -> {}", abbrev, full);
        }
    }

    #[test]
    fn test_round_trip_reverse() {
        let dict = AbbrevDict::new();
        
        // For all global mappings, expand then compress should return original abbrev
        // Skip single-letter keys as they use contextual expansion which may differ
        for (&abbrev, &_full) in dict.global_mappings() {
            if abbrev.len() == 1 {
                continue; // Single-letter keys use contextual expansion
            }
            let expanded = dict.expand(abbrev, "");
            let compressed = dict.compress(&expanded);
            assert_eq!(compressed, abbrev, "Reverse round-trip failed for {}", abbrev);
        }
    }

    #[test]
    fn test_new_v2_abbreviations() {
        let dict = AbbrevDict::new();
        
        // Test new V2 abbreviation expansions
        assert_eq!(dict.expand("v", ""), "version"); // Short alias
        assert_eq!(dict.expand("ws", ""), "workspace");
        assert_eq!(dict.expand("eds", ""), "editors");
        assert_eq!(dict.expand("repo", ""), "repository");
        assert_eq!(dict.expand("cont", ""), "container");
        assert_eq!(dict.expand("ci", ""), "ci_cd");
    }

    #[test]
    fn test_new_v2_compressions() {
        let dict = AbbrevDict::new();
        
        // Test new V2 full name compressions
        // Note: "version" compresses to "vr" (canonical), not "v" (alias)
        assert_eq!(dict.compress("version"), "vr");
        assert_eq!(dict.compress("workspace"), "ws");
        assert_eq!(dict.compress("editors"), "eds");
        assert_eq!(dict.compress("repository"), "repo");
        assert_eq!(dict.compress("container"), "cont");
        assert_eq!(dict.compress("ci_cd"), "ci");
    }
}
