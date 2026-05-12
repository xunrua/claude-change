use crate::backup::{backup_current, backup_retention_count, prune_backups, rollback};
use crate::config::{write_atomic, ConfigPaths};
use crate::error::{ProfileError, Result};
use crate::profile::{list_profiles, Profile};
use crate::validation::validate_profile;
use std::fs;

pub struct Switcher {
    pub paths: ConfigPaths,
}

impl Switcher {
    pub fn new() -> Result<Self> {
        let paths = ConfigPaths::new()?;
        paths.ensure_dirs()?;
        Ok(Self { paths })
    }

    /// List all available profiles with active marker
    pub fn list_profiles(&self) -> Result<Vec<(Profile, bool)>> {
        let active = self.get_active_profile()?;
        let profiles = list_profiles(&self.paths.profiles_dir)?;

        Ok(profiles
            .into_iter()
            .map(|p| {
                let is_active = active.as_ref().map(|a| a == &p.name).unwrap_or(false);
                (p, is_active)
            })
            .collect())
    }

    /// Switch to a profile (full-file replacement)
    pub fn switch_to(&self, profile_name: &str, dry_run: bool, force: bool) -> Result<()> {
        // Load target profile
        let profile_path = Profile::profile_path(&self.paths.profiles_dir, profile_name);
        if !profile_path.exists() {
            return Err(ProfileError::ProfileNotFound(profile_name.to_string()));
        }
        let profile = Profile::load(&profile_path)?;

        // Validate profile
        validate_profile(&profile)?;

        // Check for dirty settings
        if !force {
            self.check_dirty()?;
        }

        if dry_run {
            println!("[DRY RUN] Would switch to profile: {}", profile_name);
            println!("[DRY RUN] Settings would be written to: {}", self.paths.settings_json_path.display());
            if let Some(env) = &profile.settings.env {
                if let Some(url) = env.get("ANTHROPIC_BASE_URL") {
                    println!("[DRY RUN] Base URL: {}", url);
                }
                if let Some(key) = env.get("ANTHROPIC_AUTH_TOKEN") {
                    println!("[DRY RUN] API Key: {}...{}", &key[..key.len().min(4)], &key[key.len().saturating_sub(4)..]);
                }
            }
            return Ok(());
        }

        // Backup current settings
        if self.paths.settings_json_path.exists() {
            let backup_path = backup_current(&self.paths.settings_json_path, &self.paths.backups_dir)?;
            println!("Backup created: {}", backup_path.display());

            // Prune old backups
            let removed = prune_backups(&self.paths.backups_dir, backup_retention_count())?;
            if removed > 0 {
                println!("Pruned {} old backup(s)", removed);
            }
        }

        // Convert profile to settings JSON
        let settings_json = profile.to_settings_json()?;

        // Write atomically
        write_atomic(&self.paths.settings_json_path, settings_json.as_bytes())?;

        // Update active profile marker
        self.set_active_profile(profile_name)?;

        println!("Switched to profile: {}", profile_name);
        println!("Note: Restart Claude Code for changes to take effect.");

        Ok(())
    }

    /// Rollback to the most recent backup
    pub fn rollback(&self) -> Result<()> {
        let backup_path = rollback(&self.paths.settings_json_path, &self.paths.backups_dir)?;
        println!("Restored from backup: {}", backup_path.display());
        println!("Note: Restart Claude Code for changes to take effect.");

        // Clear active profile marker since we don't know which profile this backup corresponds to
        let _ = fs::remove_file(&self.paths.active_profile_file);

        Ok(())
    }

    /// Get the currently active profile name
    pub fn get_active_profile(&self) -> Result<Option<String>> {
        if !self.paths.active_profile_file.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&self.paths.active_profile_file)?;
        let name = content.trim().to_string();
        if name.is_empty() {
            Ok(None)
        } else {
            Ok(Some(name))
        }
    }

    /// Set the active profile name
    fn set_active_profile(&self, name: &str) -> Result<()> {
        write_atomic(&self.paths.active_profile_file, name.as_bytes())?;
        Ok(())
    }

    /// 开启或关闭 yolo 模式
    /// yolo 模式会跳过危险操作的确认提示
    pub fn set_yolo(&self, enabled: bool) -> Result<()> {
        use crate::backup::backup_current;

        // 读取当前 settings.json
        let settings_path = &self.paths.settings_json_path;
        if !settings_path.exists() {
            return Err(ProfileError::PathError(
                "未找到 settings.json，请先创建一个 profile 并切换".to_string()
            ));
        }

        // 先备份当前配置，避免 dirty check 误报
        backup_current(settings_path, &self.paths.backups_dir)?;

        let content = fs::read_to_string(settings_path)?;
        let mut settings: serde_json::Value = serde_json::from_str(&content)?;

        // 修改 skipDangerousModePermissionPrompt 字段
        if let Some(obj) = settings.as_object_mut() {
            obj.insert(
                "skipDangerousModePermissionPrompt".to_string(),
                serde_json::Value::Bool(enabled),
            );
        }

        // 写回文件
        let new_content = serde_json::to_string_pretty(&settings)?;
        write_atomic(settings_path, new_content.as_bytes())?;

        if enabled {
            println!("Yolo 模式已开启 🚀");
            println!("危险操作将不再提示确认");
        } else {
            println!("安全模式已开启 🛡️");
            println!("危险操作将需要确认");
        }
        println!("重启 Claude Code 后生效");

        Ok(())
    }

    /// Check if settings.json has been modified outside the tool
    fn check_dirty(&self) -> Result<()> {
        if !self.paths.settings_json_path.exists() {
            return Ok(());
        }

        let current_settings = fs::read_to_string(&self.paths.settings_json_path)?;

        // Compare with the most recent backup
        if let Ok(backup) = crate::backup::get_most_recent_backup(&self.paths.backups_dir) {
            let backup_content = fs::read_to_string(&backup)?;
            if current_settings.trim() != backup_content.trim() {
                return Err(ProfileError::DirtySettings);
            }
        }

        Ok(())
    }
}
