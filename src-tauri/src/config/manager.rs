use crate::errors::VaultError;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{path::BaseDirectory, App, Manager};
use tauri::{AppHandle, Error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vault {
    pub name: String,
    pub path: PathBuf,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub vaults: Vec<Vault>,

    #[serde(default)]
    pub active_vault: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            vaults: vec![],
            active_vault: None,
        }
    }
}

pub fn default_vault(app_handle: &tauri::AppHandle) -> Result<PathBuf, tauri::Error> {
    app_handle
        .path()
        .resolve("Monalisa Vault", BaseDirectory::Document)
}

pub fn get_config_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, VaultError> {
    return app_handle
        .path()
        .resolve(".monalisa/config.json", BaseDirectory::Config)
        .map_err(|e| VaultError::PathResolution(e.to_string()));
}
#[tauri::command]
pub fn load_config(app_handle: tauri::AppHandle) -> Result<AppConfig, VaultError> {
    let path = get_config_path(&app_handle)?;
    println!("Config-Pfad: {:?}", path);
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}
pub fn save_config(config: &AppConfig, app_handle: tauri::AppHandle) {
    let path = get_config_path(&app_handle).unwrap();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    let content = serde_json::to_string_pretty(config).unwrap();
    fs::write(path, content).ok();
}
