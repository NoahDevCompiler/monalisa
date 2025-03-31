use std::fmt::format;
use std::fs;
use std::path::{
    Path,
    PathBuf
};
use std::str::SplitWhitespace;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use notify::{
    Config,
    RecommendedWatcher,
    Watcher,
    RecursiveMode
};
use std::sync::mpsc::channel;
use std::thread::{park, spawn};

//global vault value for the active or last active vault path


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
pub fn create_folder(app_handle: AppHandle, name: Option<&str>, folder: Option<&str>) -> Result<(), String> {
    let mut vault_path = get_default_vault_path(&app_handle)?;

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
pub fn create_md_file(app_handle: AppHandle, name: Option<&str>, folder: Option<&str>) -> Result<(), String> {
    let mut vault_path = get_default_vault_path(&app_handle)?;

    if let Some(folder_name) = folder {
        vault_path = vault_path.join(folder_name)
    }
    //if file exists err, creation Error

    let file_name = match name{
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return Err(format!("File name empty")),
    };

    let file_name = if file_name.ends_with(".md"){
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

pub fn sync_vault(path: PathBuf) {
    let (tx, rx) = channel();
    
    let path = path.to_path_buf();
    spawn(move ||{
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
        watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        
        loop {
            park();
        }
    });

}
