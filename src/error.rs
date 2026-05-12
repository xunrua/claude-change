use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),

    #[error("Failed to parse profile TOML: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Failed to serialize profile: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Settings JSON error: {0}")]
    SettingsJsonError(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid API key format for provider '{0}'")]
    InvalidApiKey(String),

    #[error("Profile '{0}' already exists")]
    ProfileAlreadyExists(String),

    #[error("No backups found")]
    NoBackups,

    #[error("Settings file has been modified outside the tool. Use --force to override")]
    DirtySettings,

    #[error("Another switch operation is in progress")]
    ConcurrentSwitch,

    #[error("Failed to parse existing settings.json: {0}")]
    SettingsParseError(String),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Hook error: {0}")]
    HookError(String),
}

pub type Result<T> = std::result::Result<T, ProfileError>;
