//! Configuration loader for Antares Auto Tune Slim.
//!
//! Reads settings from the user profile under %LOCALAPPDATA%\Antares Auto Tune Slim\config.toml,
//! falling back to compiled defaults when the file is missing or malformed.

use std::path::PathBuf;

/// Default path under %LOCALAPPDATA% where runtime settings live.
pub const CONFIG_DIR: &str = "Antares Auto Tune Slim";

/// Settings version. Bumped when the on-disk schema changes.
pub const SCHEMA_VERSION: u32 = 3;

#[derive(Debug, Clone)]
pub struct Settings {
    pub auto_update: bool,
    pub telemetry: bool,
    pub ui_scale: f32,
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_update: true,
            telemetry: false,
            ui_scale: 1.25,
            log_level: String::from("debug"),
        }
    }
}

pub fn config_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from("."));
    PathBuf::from(base).join(CONFIG_DIR).join("config.toml")
}

pub fn load() -> Settings {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(_body) => Settings::default(),  // real parser omitted in this snapshot
        Err(_) => Settings::default(),
    }
}
