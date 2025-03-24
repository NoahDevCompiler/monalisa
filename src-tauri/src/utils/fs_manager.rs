use std::fmt::format;
use std::fs;
use std::path::PathBuf;
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
#[tauri::command]
pub fn create_md_file(app_handle: AppHandle, name: Option<&str>, folder: Option<&str>) -> Result<(), String> {
    let mut vault_path = get_default_vault_path(&app_handle)?;

    if let Some(folder_name) = folder {
        vault_path = vault_path.join(folder_name)
    }
    //if file exists err, creation Error

    let file_path = match name{
        Some(n) => vault_path.join(n),
        None => return Err(format!("File name empty")),
    };

    if let Err(e) = fs::File::create(&file_path) {
        return Err(format!("Couldnt create File {}", e));
    }
    Ok(())
}

//pub fn save_active_file() -> {}
