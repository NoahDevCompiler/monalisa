use std::path::PathBuf;
use std::fs;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

pub fn get_default_vault_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let vault_path = app_handle
        .path()
        .resolve("Monalisa Vault", BaseDirectory::Document);

    let vault_path = match vault_path {
        Ok(path) => path,  
        Err(e) => return Err(format!("Error calling path: {}", e)), 
    };

    if !vault_path.exists() {
        if let Err(e) = fs::create_dir(&vault_path) {
            return Err(format!("Couldnt create vault {}", e));
        }
    }
    Ok(vault_path)
}

#[tauri::command]
pub fn create_folder(app_handle: AppHandle, name: Option<&str>) -> Result<(), String> {
    let vault_path = get_default_vault_path(&app_handle)?;

    let folder_path = match name {
        Some(n) => vault_path.join(n),
        None => vault_path,
    };
    if !folder_path.exists() {
        if let Err(e) = fs::create_dir(&folder_path) {
            return Err(format!("Couldnt create folder {}", e));
        }
    }

    Ok(())
}
