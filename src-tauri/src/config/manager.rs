use crate::errors::VaultError;
use serde::{Deserialize, Serialize};
use tauri_plugin_dialog::init;
use std::any::type_name;
use std::fmt::format;
use std::path::{Path, PathBuf};
use std::{fs, vec};
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

fn update_config<F>(app_handle: &tauri::AppHandle, modify: F)
where
    F: FnOnce(&mut AppConfig),
{
    let path = get_config_path(app_handle).unwrap();

    let mut config: AppConfig = if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    };

    modify(&mut config);

    init_config(&app_handle);

    let content = serde_json::to_string_pretty(&config)
        .unwrap_or_else(|_| serde_json::to_string_pretty(&AppConfig::default()).unwrap());

    fs::write(path, content).ok();
}

pub fn default_vault(app_handle: &tauri::AppHandle) -> Result<PathBuf, tauri::Error> {
    app_handle
        .path()
        .resolve("Monalisa Vault", BaseDirectory::Document)
}
pub fn init_config(app_handle: &tauri::AppHandle) {
    let path = get_config_path(&app_handle).unwrap();
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
    }
    let content = serde_json::to_string(&AppConfig::default()).unwrap();
    fs::write(path, content).ok();
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
#[tauri::command]
pub fn config_add_vault(path: PathBuf, name: String, app_handle: tauri::AppHandle) {
    update_config(&app_handle, |config| {
        config.vaults.push(new_vault);
    });
}
pub fn config_set_active_vault(path: PathBuf, app_handle: &tauri::AppHandle) {
    update_config(app_handle, |config| {
        config.active_vault = Some(path);
    });
}
