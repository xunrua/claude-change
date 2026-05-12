use crate::error::{ProfileError, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ConfigPaths {
    pub profiles_dir: PathBuf,
    pub backups_dir: PathBuf,
    pub active_profile_file: PathBuf,
    pub settings_json_path: PathBuf,
    pub hooks_dir: PathBuf,
}

impl ConfigPaths {
    pub fn new() -> Result<Self> {
        let config_dir = Self::config_dir()?;
        let data_dir = Self::data_dir()?;
        let claude_config_dir = Self::claude_config_dir()?;

        Ok(Self {
            profiles_dir: config_dir.join("profiles"),
            backups_dir: data_dir.join("backups"),
            active_profile_file: data_dir.join("active_profile"),
            settings_json_path: claude_config_dir.join("settings.json"),
            hooks_dir: claude_config_dir.join("hooks"),
        })
    }

    fn config_dir() -> Result<PathBuf> {
        if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
            Ok(PathBuf::from(dir).join("claude-profile"))
        } else if let Some(dir) = dirs::config_dir() {
            Ok(dir.join("claude-profile"))
        } else {
            Err(ProfileError::PathError("Cannot find config directory".to_string()))
        }
    }

    fn data_dir() -> Result<PathBuf> {
        if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
            Ok(PathBuf::from(dir).join("claude-profile"))
        } else if let Some(dir) = dirs::data_dir() {
            Ok(dir.join("claude-profile"))
        } else {
            Err(ProfileError::PathError("Cannot find data directory".to_string()))
        }
    }

    fn claude_config_dir() -> Result<PathBuf> {
        if let Ok(dir) = std::env::var("CLAUDE_CONFIG_DIR") {
            Ok(PathBuf::from(dir))
        } else if let Some(home) = dirs::home_dir() {
            Ok(home.join(".claude"))
        } else {
            Err(ProfileError::PathError("Cannot find Claude config directory".to_string()))
        }
    }

    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.profiles_dir)?;
        std::fs::create_dir_all(&self.backups_dir)?;
        Ok(())
    }
}

pub fn write_atomic(path: &Path, content: &[u8]) -> Result<()> {
    let temp_path = path.with_extension("tmp");
    std::fs::write(&temp_path, content)?;
    std::fs::rename(&temp_path, path)?;
    Ok(())
}
