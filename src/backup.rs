use crate::error::{ProfileError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Create a timestamped backup of the current settings.json
pub fn backup_current(settings_path: &Path, backups_dir: &Path) -> Result<PathBuf> {
    if !settings_path.exists() {
        return Err(ProfileError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Settings file not found: {}", settings_path.display()),
        )));
    }

    fs::create_dir_all(backups_dir)?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("settings_backup_{}.json", timestamp);
    let backup_path = backups_dir.join(&backup_name);

    fs::copy(settings_path, &backup_path)?;

    // Set restrictive permissions on backup
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&backup_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&backup_path, perms)?;
    }

    Ok(backup_path)
}

/// Get the most recent backup, sorted by filename (which includes timestamp)
pub fn get_most_recent_backup(backups_dir: &Path) -> Result<PathBuf> {
    if !backups_dir.exists() {
        return Err(ProfileError::NoBackups);
    }

    let mut backups: Vec<PathBuf> = fs::read_dir(backups_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()).map(|e| e == "json").unwrap_or(false))
        .collect();

    if backups.is_empty() {
        return Err(ProfileError::NoBackups);
    }

    // Sort by filename (descending - most recent first)
    backups.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    Ok(backups[0].clone())
}

/// Restore the most recent backup to settings.json
pub fn rollback(settings_path: &Path, backups_dir: &Path) -> Result<PathBuf> {
    let backup_path = get_most_recent_backup(backups_dir)?;
    fs::copy(&backup_path, settings_path)?;
    Ok(backup_path)
}

/// Prune old backups, keeping only the most recent N
pub fn prune_backups(backups_dir: &Path, keep_count: usize) -> Result<usize> {
    if !backups_dir.exists() || keep_count == 0 {
        return Ok(0);
    }

    let mut backups: Vec<PathBuf> = fs::read_dir(backups_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("settings_backup_"))
                .unwrap_or(false)
        })
        .collect();

    if backups.len() <= keep_count {
        return Ok(0);
    }

    // Sort by filename (descending - most recent first)
    backups.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    let mut removed = 0;
    for backup in backups.iter().skip(keep_count) {
        fs::remove_file(backup)?;
        removed += 1;
    }

    Ok(removed)
}

/// Get backup retention count from env or default
pub fn backup_retention_count() -> usize {
    std::env::var("CLAUDE_PROFILE_BACKUP_COUNT").ok().and_then(|s| s.parse().ok()).unwrap_or(50)
}
