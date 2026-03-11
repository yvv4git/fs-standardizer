use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub rules: Vec<RenameRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenameRule {
    pub pattern: String,
    pub replacement: String,
}

impl AppConfig {
    pub fn default_config() -> Self {
        Self { rules: vec![] }
    }

    pub fn load(path: &str) -> Result<Self, String> {
        if !std::path::Path::new(path).exists() {
            return Ok(AppConfig::default_config());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        let config: AppConfig = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(config)
    }
}
