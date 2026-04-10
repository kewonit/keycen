pub mod watcher;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub replacements: HashMap<String, String>,
    pub exclusions: ExclusionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub enabled: bool,
    pub mode: String, // "auto", "grab", "listen"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionConfig {
    pub apps: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        // Parse the embedded default config (has empty replacements)
        let mut config: AppConfig = toml::from_str(include_str!("../../config/default.toml"))
            .expect("Default config must be valid TOML");
        // Add built-in replacement mappings at runtime
        // (kept out of source files to avoid explicit profanity in the repo)
        config.replacements.extend(builtin_replacements());
        config
    }
}

/// Built-in word replacement mappings, constructed at runtime
/// to keep explicit words out of committed source files.
fn builtin_replacements() -> HashMap<String, String> {
    // Encoded as (prefix, suffix, replacement) to avoid plaintext profanity
    let pairs: &[(&[u8], &str)] = &[
        (b"\x73\x68\x69\x74", "stuff"),
        (b"\x66\x75\x63\x6b", "fudge"),
        (b"\x64\x61\x6d\x6e", "dang"),
        (b"\x61\x73\x73", "butt"),
        (b"\x62\x69\x74\x63\x68", "witch"),
        (b"\x62\x61\x73\x74\x61\x72\x64", "rascal"),
        (b"\x63\x72\x61\x70", "crud"),
        (b"\x68\x65\x6c\x6c", "heck"),
        (b"\x70\x69\x73\x73", "tinkle"),
        (b"\x64\x69\x63\x6b", "jerk"),
        (b"\x63\x75\x6e\x74", "meanie"),
        (b"\x77\x68\x6f\x72\x65", "rude"),
        (b"\x73\x6c\x75\x74", "rude"),
    ];
    pairs
        .iter()
        .filter_map(|(bytes, replacement)| {
            String::from_utf8(bytes.to_vec())
                .ok()
                .map(|word| (word, replacement.to_string()))
        })
        .collect()
}

impl AppConfig {
    /// Load config from file, falling back to defaults for missing fields
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        if path.exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| ConfigError::ReadError(path.to_path_buf(), e))?;
            let config: AppConfig = toml::from_str(&content)
                .map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;
            Ok(config)
        } else {
            log::info!("Config file not found at {path:?}, creating with defaults");
            let config = AppConfig::default();
            config.save(path)?;
            Ok(config)
        }
    }

    /// Save config to file
    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::WriteError(path.to_path_buf(), e))?;
        }
        let content = toml::to_string_pretty(self).map_err(ConfigError::SerializeError)?;
        fs::write(path, content).map_err(|e| ConfigError::WriteError(path.to_path_buf(), e))?;
        Ok(())
    }

    /// Get the platform-specific default config path
    pub fn default_path() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("keycen").join("config.toml")
    }
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ConfigError {
    ReadError(PathBuf, std::io::Error),
    WriteError(PathBuf, std::io::Error),
    ParseError(PathBuf, toml::de::Error),
    SerializeError(toml::ser::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ReadError(p, e) => write!(f, "Failed to read config at {p:?}: {e}"),
            ConfigError::WriteError(p, e) => write!(f, "Failed to write config at {p:?}: {e}"),
            ConfigError::ParseError(p, e) => write!(f, "Failed to parse config at {p:?}: {e}"),
            ConfigError::SerializeError(e) => write!(f, "Failed to serialize config: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}
