use crate::switcher::Switcher;
use crate::profile::{mask_api_key, Profile};
use crate::error::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-profile")]
#[command(about = "Manage Claude Code configuration profiles")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available profiles
    List,

    /// Switch to a profile
    Switch {
        /// Profile name
        name: String,

        /// Show what would happen without making changes
        #[arg(long)]
        dry_run: bool,

        /// Skip dirty check
        #[arg(long)]
        force: bool,
    },

    /// Rollback to the previous configuration
    Rollback,

    /// Import an existing settings.json as a profile
    Import {
        /// Path to settings.json file
        file: String,

        /// Profile name
        #[arg(long, short)]
        name: String,
    },

    /// Validate a profile without switching
    Validate {
        /// Profile name
        name: String,
    },

    /// Show profile details (API keys masked)
    Show {
        /// Profile name
        name: String,
    },

    /// Create a new profile template
    Add {
        /// Profile name
        name: String,
    },

    /// Remove a profile
    Remove {
        /// Profile name
        name: String,

        /// Skip confirmation
        #[arg(long)]
        yes: bool,
    },

    /// Launch TUI (if compiled with tui feature)
    #[cfg(feature = "tui")]
    Tui,

    /// Hook management commands
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },

    /// 开启 yolo 模式（跳过危险操作确认提示）
    Yolo,

    /// 关闭 yolo 模式（恢复安全模式，危险操作需要确认）
    Safe,
}

#[derive(Subcommand)]
pub enum HookAction {
    /// Install the Claude Code hook
    Install,
    /// Uninstall the Claude Code hook
    Uninstall,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let switcher = Switcher::new()?;

    match cli.command {
        Commands::List => cmd_list(&switcher),
        Commands::Switch { name, dry_run, force } => {
            switcher.switch_to(&name, dry_run, force)
        }
        Commands::Rollback => switcher.rollback(),
        Commands::Import { file, name } => cmd_import(&switcher, &file, &name),
        Commands::Validate { name } => cmd_validate(&switcher, &name),
        Commands::Show { name } => cmd_show(&switcher, &name),
        Commands::Add { name } => cmd_add(&switcher, &name),
        Commands::Remove { name, yes } => cmd_remove(&switcher, &name, yes),
        #[cfg(feature = "tui")]
        Commands::Tui => {
            // 启动 TUI 交互界面
            crate::tui::run_tui()
        }
        Commands::Hook { action } => match action {
            HookAction::Install => cmd_hook_install(&switcher),
            HookAction::Uninstall => cmd_hook_uninstall(&switcher),
        },
        Commands::Yolo => switcher.set_yolo(true),
        Commands::Safe => switcher.set_yolo(false),
    }
}

fn cmd_list(switcher: &Switcher) -> Result<()> {
    let profiles = switcher.list_profiles()?;

    if profiles.is_empty() {
        println!("No profiles found.");
        println!("Create one with: claude-profile add <name>");
        println!("Or import an existing settings.json: claude-profile import <file> --name <name>");
        return Ok(());
    }

    println!("{:<12} {:<20} {}", "ACTIVE", "NAME", "DESCRIPTION");
    println!("{}", "-".repeat(60));

    for (profile, is_active) in profiles {
        let active_marker = if is_active { "*" } else { " " };
        let desc = profile.description.as_deref().unwrap_or("-");
        let name_display = if profile.name.len() > 18 {
            format!("{}..", &profile.name[..16])
        } else {
            profile.name.clone()
        };
        println!("{:<12} {:<20} {}", active_marker, name_display, desc);
    }

    Ok(())
}

fn cmd_import(switcher: &Switcher, file: &str, name: &str) -> Result<()> {
    let content = std::fs::read_to_string(file)?;
    let profile = Profile::from_settings_json(name, &content)?;

    let profile_path = Profile::profile_path(&switcher.paths.profiles_dir, name);
    if profile_path.exists() {
        return Err(crate::error::ProfileError::ProfileAlreadyExists(name.to_string()));
    }

    profile.save(&profile_path)?;
    println!("Imported profile '{}' from {}", name, file);
    println!("Profile saved to: {}", profile_path.display());

    Ok(())
}

fn cmd_validate(switcher: &Switcher, name: &str) -> Result<()> {
    let profile_path = Profile::profile_path(&switcher.paths.profiles_dir, name);
    if !profile_path.exists() {
        return Err(crate::error::ProfileError::ProfileNotFound(name.to_string()));
    }

    let profile = Profile::load(&profile_path)?;
    crate::validation::validate_profile(&profile)?;

    println!("Profile '{}' is valid.", name);
    Ok(())
}

fn cmd_show(switcher: &Switcher, name: &str) -> Result<()> {
    let profile_path = Profile::profile_path(&switcher.paths.profiles_dir, name);
    if !profile_path.exists() {
        return Err(crate::error::ProfileError::ProfileNotFound(name.to_string()));
    }

    let profile = Profile::load(&profile_path)?;
    let content = toml::to_string_pretty(&profile)?;

    // Mask API keys in the output
    let masked: String = content
        .lines()
        .map(|line| {
            if line.contains("ANTHROPIC_AUTH_TOKEN") {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let value = parts[1].trim().trim_matches('"');
                    format!("{} = \"{}\"", parts[0].trim(), mask_api_key(value))
                } else {
                    line.to_string()
                }
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", masked);
    Ok(())
}

fn cmd_add(switcher: &Switcher, name: &str) -> Result<()> {
    let profile_path = Profile::profile_path(&switcher.paths.profiles_dir, name);
    if profile_path.exists() {
        return Err(crate::error::ProfileError::ProfileAlreadyExists(name.to_string()));
    }

    let profile = Profile {
        name: name.to_string(),
        description: Some("New profile".to_string()),
        settings: Default::default(),
    };

    profile.save(&profile_path)?;
    println!("Created profile '{}'", name);
    println!("Edit it at: {}", profile_path.display());

    Ok(())
}

fn cmd_remove(switcher: &Switcher, name: &str, yes: bool) -> Result<()> {
    let profile_path = Profile::profile_path(&switcher.paths.profiles_dir, name);
    if !profile_path.exists() {
        return Err(crate::error::ProfileError::ProfileNotFound(name.to_string()));
    }

    if !yes {
        println!("Are you sure you want to remove profile '{}'? [y/N]", name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    std::fs::remove_file(&profile_path)?;
    println!("Removed profile '{}'", name);
    Ok(())
}

/// 安装 Claude Code hook
/// 将 hook 脚本复制到 ~/.claude/hooks/ 目录
fn cmd_hook_install(switcher: &Switcher) -> Result<()> {
    // 获取 Claude Code 配置目录
    let claude_dir = switcher.paths.settings_json_path
        .parent()
        .ok_or_else(|| crate::error::ProfileError::PathError(
            "无法获取 Claude Code 配置目录".to_string()
        ))?;

    crate::hook::install_hook(claude_dir)
}

/// 卸载 Claude Code hook
/// 从 ~/.claude/hooks/ 目录移除 hook 脚本
fn cmd_hook_uninstall(switcher: &Switcher) -> Result<()> {
    let claude_dir = switcher.paths.settings_json_path
        .parent()
        .ok_or_else(|| crate::error::ProfileError::PathError(
            "无法获取 Claude Code 配置目录".to_string()
        ))?;

    crate::hook::uninstall_hook(claude_dir)
}
