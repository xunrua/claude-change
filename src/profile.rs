use crate::error::Result;
use crate::settings::ClaudeSettings;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A Profile defines a complete Claude Code configuration.
/// The `[settings]` section maps 1:1 to settings.json keys.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Profile {
    /// Profile name (used as filename stem and identifier)
    pub name: String,

    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The complete settings configuration
    #[serde(default)]
    pub settings: ClaudeSettings,
}

impl Profile {
    /// Load a profile from a TOML file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut profile: Self = toml::from_str(&content)?;
        // Derive name from filename stem if not set in the file
        #[allow(clippy::collapsible_if)]
        if profile.name.is_empty() {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                profile.name = stem.to_string();
            }
        }
        Ok(profile)
    }

    /// Save profile to a TOML file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        crate::config::write_atomic(path, content.as_bytes())?;
        Ok(())
    }

    /// Get the profile file path given a profiles directory and name
    pub fn profile_path(profiles_dir: &Path, name: &str) -> PathBuf {
        profiles_dir.join(format!("{}.toml", name))
    }

    /// Convert this profile to a JSON string (for writing to settings.json)
    pub fn to_settings_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.settings)?)
    }

    /// Create a Profile from existing settings.json content
    pub fn from_settings_json(name: &str, json_content: &str) -> Result<Self> {
        let settings: ClaudeSettings = serde_json::from_str(json_content)?;
        Ok(Self { name: name.to_string(), description: None, settings })
    }
}

/// List all available profiles in the profiles directory
pub fn list_profiles(profiles_dir: &Path) -> Result<Vec<Profile>> {
    let mut profiles = Vec::new();
    if !profiles_dir.exists() {
        return Ok(profiles);
    }
    for entry in std::fs::read_dir(profiles_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            match Profile::load(&path) {
                Ok(profile) => profiles.push(profile),
                Err(e) => eprintln!("Warning: Failed to load profile {:?}: {}", path, e),
            }
        }
    }
    Ok(profiles)
}

/// Mask sensitive values (API keys) for display
pub fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        "***".to_string()
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
