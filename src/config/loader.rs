use anyhow::{Result, anyhow};
use std::fs;
use std::path::PathBuf;

use crate::config::settings::Settings;

pub fn load_settings() -> Result<Settings> {
    let path = config_path()?;

    if !path.exists() {
        create_default_config(&path)?;
    }

    let content = fs::read_to_string(&path)?;

    let settings: Settings = serde_json::from_str(&content)?;

    Ok(settings)
}

fn create_default_config(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let settings = Settings::default();

    let json = serde_json::to_string_pretty(&settings)?;

    fs::write(path, json)?;

    Ok(())
}

pub fn config_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("failed to resolve config directory"))?;

    Ok(config_dir.join("clipforge").join("settings.json"))
}
