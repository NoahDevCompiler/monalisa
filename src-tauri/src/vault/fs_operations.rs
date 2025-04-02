use tauri::{AppHandle, Manager};
use std::fs;
use crate::state::SharedState;

#[tauri::command]
pub fn create_folder(app_handle: AppHandle, name: Option<&str>, folder: Option<&str>) -> Result<(), String> {
    
    let state = app_handle.state::<SharedState>();
    let mut vault_path = state.lock()?.current_vault.as_ref().ok_or(Va)

    if let Some(folder_name) = folder {
        vault_path = vault_path.join(folder_name)
    }

    let folder_path = match name {
        Some(n) => vault_path.join(n),
        None => vault_path,
    };
    if folder_path.exists() {
        return Err("Folder already exists".to_string());
    }
    if let Err(e) = fs::create_dir(&folder_path) {
        return Err(format!("Couldnt create folder {}", e));
    }

    Ok(())
}
#[tauri::command]
pub fn create_md_file(
    app_handle: AppHandle,
    name: Option<&str>,
    folder: Option<&str>,
) -> Result<(), String> {
    let mut vault_path = get_default_vault_path(&app_handle)?;

    if let Some(folder_name) = folder {
        vault_path = vault_path.join(folder_name)
    }

    let file_name = match name {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return Err(format!("File name empty")),
    };

    let file_name = if file_name.ends_with(".md") {
        file_name.to_string()
    } else {
        format!("{}.md", file_name)
    };
    let file_path = vault_path.join(file_name);

    if file_path.exists() {
        return Err("File already exists".to_string());
    }
    fs::File::create(&file_path).map_err(|e| format!("Couldnt create file {}", e))?;
    Ok(())
}