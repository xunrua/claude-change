// 集成测试
// 测试 profile 的完整生命周期：创建、导入、切换、回滚

use std::fs;
use tempfile::TempDir;

/// 测试 Profile 的 TOML 序列化和反序列化
#[test]
fn test_profile_roundtrip() {
    use claude_profile::profile::Profile;
    use claude_profile::settings::ClaudeSettings;

    // 创建一个临时目录
    let temp_dir = TempDir::new().unwrap();
    let profile_path = temp_dir.path().join("test.toml");

    // 创建一个测试 profile
    let mut env = std::collections::HashMap::new();
    env.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.example.com".to_string());
    env.insert("ANTHROPIC_AUTH_TOKEN".to_string(), "sk-test-key".to_string());

    let profile = Profile {
        name: "test".to_string(),
        description: Some("测试 profile".to_string()),
        settings: ClaudeSettings {
            env: Some(env),
            model: Some("opus".to_string()),
            effort_level: Some("xhigh".to_string()),
            language: Some("简体中文".to_string()),
            ..Default::default()
        },
    };

    // 保存到文件
    profile.save(&profile_path).unwrap();

    // 从文件加载
    let loaded = Profile::load(&profile_path).unwrap();

    // 验证数据一致性
    assert_eq!(loaded.name, "test");
    assert_eq!(loaded.description, Some("测试 profile".to_string()));
    assert_eq!(loaded.settings.model, Some("opus".to_string()));
    assert_eq!(loaded.settings.effort_level, Some("xhigh".to_string()));
    assert_eq!(loaded.settings.language, Some("简体中文".to_string()));

    let env = loaded.settings.env.unwrap();
    assert_eq!(env.get("ANTHROPIC_BASE_URL"), Some(&"https://api.example.com".to_string()));
    assert_eq!(env.get("ANTHROPIC_AUTH_TOKEN"), Some(&"sk-test-key".to_string()));
}

/// 测试从 settings.json 导入 profile
#[test]
fn test_profile_from_settings_json() {
    use claude_profile::profile::Profile;

    // 模拟一个 settings.json 内容
    let json_content = r#"{
        "env": {
            "ANTHROPIC_BASE_URL": "https://api.kimi.com/coding/",
            "ANTHROPIC_AUTH_TOKEN": "sk-test-key"
        },
        "model": "opus[1m]",
        "effortLevel": "xhigh",
        "language": "简体中文"
    }"#;

    let profile = Profile::from_settings_json("kimi", json_content).unwrap();

    assert_eq!(profile.name, "kimi");
    assert_eq!(profile.settings.model, Some("opus[1m]".to_string()));
    assert_eq!(profile.settings.effort_level, Some("xhigh".to_string()));
    assert_eq!(profile.settings.language, Some("简体中文".to_string()));
}

/// 测试备份创建和回滚
#[test]
fn test_backup_and_rollback() {
    use claude_profile::backup::{backup_current, get_most_recent_backup, rollback};

    let temp_dir = TempDir::new().unwrap();
    let settings_path = temp_dir.path().join("settings.json");
    let backups_dir = temp_dir.path().join("backups");

    // 创建初始 settings.json
    let original_content = "{\"model\": \"original\"}";
    fs::write(&settings_path, original_content).unwrap();

    // 创建备份
    let backup_path = backup_current(&settings_path, &backups_dir).unwrap();
    assert!(backup_path.exists());

    // 修改 settings.json
    let new_content = "{\"model\": \"new\"}";
    fs::write(&settings_path, new_content).unwrap();

    // 验证修改后的内容
    let current = fs::read_to_string(&settings_path).unwrap();
    assert_eq!(current, new_content);

    // 回滚
    rollback(&settings_path, &backups_dir).unwrap();

    // 验证回滚后的内容
    let restored = fs::read_to_string(&settings_path).unwrap();
    assert_eq!(restored, original_content);

    // 验证可以获取最新的备份
    let recent = get_most_recent_backup(&backups_dir).unwrap();
    assert!(recent.exists());
}

/// 测试备份保留策略
#[test]
fn test_backup_retention() {
    use claude_profile::backup::prune_backups;

    let temp_dir = TempDir::new().unwrap();
    let backups_dir = temp_dir.path().join("backups");
    fs::create_dir_all(&backups_dir).unwrap();

    // 创建 5 个备份文件
    for i in 0..5 {
        let name = format!("settings_backup_20260512_{:02}0000.json", i);
        fs::write(backups_dir.join(&name), format!("{{\"idx\": {}}}", i)).unwrap();
    }

    // 保留 3 个
    let removed = prune_backups(&backups_dir, 3).unwrap();
    assert_eq!(removed, 2);

    // 验证只剩 3 个
    let remaining: Vec<_> = fs::read_dir(&backups_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(remaining.len(), 3);
}

/// 测试原子写入
#[test]
fn test_atomic_write() {
    use claude_profile::config::write_atomic;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    write_atomic(&file_path, b"hello world").unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "hello world");
}

/// 测试 API key 掩码
#[test]
fn test_mask_api_key() {
    use claude_profile::profile::mask_api_key;

    // 长 key
    assert_eq!(mask_api_key("sk-abcdefghijklmnopqrstuvwxyz"), "sk-a...wxyz");

    // 短 key（<= 8 字符返回 ***）
    assert_eq!(mask_api_key("sk-abc"), "***");

    // 超短 key
    assert_eq!(mask_api_key("abc"), "***");
}

/// 测试配置路径解析
#[test]
fn test_config_paths() {
    use claude_profile::config::ConfigPaths;

    // 在测试环境中，应该能成功创建 ConfigPaths
    let paths = ConfigPaths::new().unwrap();

    // 验证路径不为空
    assert!(!paths.profiles_dir.as_os_str().is_empty());
    assert!(!paths.backups_dir.as_os_str().is_empty());
    assert!(!paths.settings_json_path.as_os_str().is_empty());
}
