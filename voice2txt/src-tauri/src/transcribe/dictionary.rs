use anyhow::Result;
use std::path::Path;

/// A single dictionary correction entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DictionaryEntry {
    /// What Whisper might transcribe (e.g., "react native")
    pub from: String,
    /// What it should be corrected to (e.g., "React Native")
    pub to: String,
}

/// Custom dictionary for correcting developer-specific terms.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dictionary {
    entries: Vec<DictionaryEntry>,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl Dictionary {
    /// Load dictionary from a JSON file.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let dict: Dictionary = serde_json::from_str(&content)?;
        Ok(dict)
    }

    /// Save dictionary to a JSON file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a Dictionary from a list of entries.
    pub fn from_entries(entries: Vec<DictionaryEntry>) -> Self {
        Self { entries }
    }

    /// Get all entries.
    pub fn entries(&self) -> Vec<DictionaryEntry> {
        self.entries.clone()
    }

    /// Apply dictionary corrections to transcribed text.
    pub fn correct(&self, text: &str) -> String {
        let mut result = text.to_string();
        for entry in &self.entries {
            // Case-insensitive replacement
            result = result.replace(&entry.from, &entry.to);
            // Also try lowercase match
            let lower = entry.from.to_lowercase();
            result = result.replace(&lower, &entry.to);
        }
        result
    }
}
