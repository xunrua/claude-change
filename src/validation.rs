use crate::error::{ProfileError, Result};
use crate::profile::Profile;
use url::Url;

/// Validate a profile before switching
pub fn validate_profile(profile: &Profile) -> Result<()> {
    // Validate env section
    if let Some(env) = &profile.settings.env {
        // Validate base_url if present
        if let Some(base_url) = env.get("ANTHROPIC_BASE_URL") {
            validate_url(base_url)?;
        }

        // Validate API key format if present
        if let Some(api_key) = env.get("ANTHROPIC_AUTH_TOKEN") {
            validate_api_key(api_key)?;
        }
    }

    // Validate model if present
    if let Some(model) = &profile.settings.model
        && model.is_empty()
    {
        return Err(ProfileError::InvalidApiKey("Model cannot be empty".to_string()));
    }

    Ok(())
}

fn validate_url(url_str: &str) -> Result<()> {
    Url::parse(url_str).map_err(|e| ProfileError::InvalidUrl(format!("{}: {}", url_str, e)))?;
    Ok(())
}

fn validate_api_key(key: &str) -> Result<()> {
    // Check for common API key prefixes
    let valid_prefixes = ["sk-", "sk-ant-", "sk-ant-api03-"];
    let has_valid_prefix = valid_prefixes.iter().any(|prefix| key.starts_with(prefix));

    if !has_valid_prefix && key.len() < 10 {
        return Err(ProfileError::InvalidApiKey(format!(
            "API key should start with 'sk-' or 'sk-ant-' and be at least 10 characters long (got {} chars)",
            key.len()
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://api.anthropic.com").is_ok());
        assert!(validate_url("https://api.kimi.com/coding/").is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(validate_url("not-a-url").is_err());
        assert!(validate_url("").is_err());
    }

    #[test]
    fn test_validate_api_key_valid() {
        assert!(validate_api_key("sk-test-1234567890").is_ok());
        assert!(validate_api_key("sk-ant-api03-test1234567890").is_ok());
    }

    #[test]
    fn test_validate_api_key_invalid() {
        assert!(validate_api_key("short").is_err());
        assert!(validate_api_key("").is_err());
    }
}
