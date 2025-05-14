use crate::config::manager::{save_config, AppConfig};
use crate::state::ConfigState;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub fn get_config(state: State<ConfigState>) -> AppConfig {
    state.0.lock().unwrap().clone()
}
#[tauri::command]
pub fn set_active_vault(app_handle: tauri::AppHandle, path: PathBuf, state: State<ConfigState>) {
    let mut config = state.0.lock().unwrap();
    config.active_vault = Some(path);
    save_config(&config, app_handle);
}
#[tauri::command]
pub fn get_active_vault_path(state: State<ConfigState>) -> Result<PathBuf, String> {
    let config = state.0.lock().unwrap();
    config
        .active_vault
        .clone()
        .ok_or("No active vault set".to_string())
}
