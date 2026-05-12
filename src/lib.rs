pub mod backup;
pub mod cli;
pub mod config;
pub mod error;
pub mod hook;
pub mod profile;
pub mod settings;
pub mod switcher;
pub mod validation;

// TUI 模块只在启用 tui feature 时编译
#[cfg(feature = "tui")]
pub mod tui;
