use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mirrors the actual Claude Code settings.json structure.
/// Uses serde(flatten) to capture unknown fields for forward compatibility.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClaudeSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(rename = "effortLevel", skip_serializing_if = "Option::is_none")]
    pub effort_level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<serde_json::Value>,

    #[serde(rename = "enabledPlugins", skip_serializing_if = "Option::is_none")]
    pub enabled_plugins: Option<HashMap<String, bool>>,

    #[serde(rename = "extraKnownMarketplaces", skip_serializing_if = "Option::is_none")]
    pub extra_known_marketplaces: Option<HashMap<String, MarketplaceSource>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_line: Option<StatusLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "skipDangerousModePermissionPrompt", skip_serializing_if = "Option::is_none")]
    pub skip_dangerous_mode_permission_prompt: Option<bool>,

    /// Catch-all for any fields not explicitly defined above.
    /// This ensures forward compatibility when Claude Code adds new settings.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarketplaceSource {
    pub source: Source,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Source {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatusLine {
    #[serde(rename = "type")]
    pub status_type: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Attribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<String>,
}
