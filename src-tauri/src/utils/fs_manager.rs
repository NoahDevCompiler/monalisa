use crate::errors::VaultError;
use crate::models::fs::{DirContent, DirEntry};
use crate::vault;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::read_dir;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::str::SplitWhitespace;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::sync::{self, Arc, Mutex};
use std::thread::{park, spawn};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use walkdir::WalkDir;

struct VaultState {
    current_path: RwLock<String>,
}

pub fn get_default_vault_path(app_handle: tauri::AppHandle) -> Result<PathBuf, String> {
    let vault_path = app_handle
        .path()
        .resolve("Monalisa Vault", BaseDirectory::Document);

    let vault_path = match vault_path {
        Ok(path) => path,
        Err(e) => return Err(format!("Error calling path: {}", e)),
    };

    if let Err(e) = fs::create_dir_all(&vault_path) {
        return Err(format!("Couldnt create vault {}", e));
    }
    //let pathbuf =   PathBuf::new();
    //let newpath = pathbuf.join("C:\\Users\\Noah\\Music\\! Burnt Stash-Kit - 717MaKid");

    Ok(vault_path)
}

struct SerializedEvent {
    kind: String,
    paths: Vec<PathBuf>,
}

impl From<Event> for SerializedEvent {
    fn from(event: Event) -> Self {
        SerializedEvent {
            kind: format!("{:?}", event.kind),
            paths: event.paths.clone(),
        }
    }
}

#[tauri::command]
pub fn create_folder(
    app_handle: tauri::AppHandle,
    name: Option<&str>,
    folder: Option<&str>,
) -> Result<(), String> {
    let mut vault_path = get_default_vault_path(app_handle)?;

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
pub fn read_directory(app_handle: tauri::AppHandle) -> Result<DirEntry, VaultError> {
    let path = get_default_vault_path(app_handle)
        .map_err(|e| VaultError::PathResolution(e.to_string()))?;

    build_tree(&path)
}
pub fn build_tree(path: &Path) -> Result<DirEntry, VaultError> {
    let metadata = fs::metadata(path)?;
    let is_dir = metadata.is_dir();

    let mut entry = DirEntry {
        name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        path: path.to_path_buf(),
        is_dir,
        size: metadata.len(),
        children: None,
    };

    if is_dir {
        let mut children = Vec::new();
        for child in fs::read_dir(path)? {
            let child = child?;
            children.push(build_tree(&child.path())?);
        }
        entry.children = Some(children);
    }

    Ok(entry)
}

#[tauri::command]
pub fn create_md_file(
    app_handle: tauri::AppHandle,
    name: Option<&str>,
    folder: Option<&str>,
) -> Result<(), String> {
    let mut vault_path = get_default_vault_path(app_handle)?;

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

#[tauri::command]
pub fn read_file(path: PathBuf) -> Result<String, VaultError> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

#[tauri::command]
pub fn write_file(path: PathBuf, content: String) -> Result<(), VaultError> {
<<<<<<< HEAD
    let _ = fs::write(&path, content).map_err(|_e| VaultError::PermissionDenied { path: path });
    return Ok(());
=======

    let _ = fs::write(&path, content).map_err(|_e| VaultError::PermissionDenied { path: path });
    return Ok(())
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
}

//#[tauri::command]
//pub fn sync_vault(app_handle: AppHandle, path: PathBuf) {
//    let (tx, rx) = channel();
//
//    let path = path.to_path_buf();
//    spawn(move || {
//        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
//        watcher.watch(&path, RecursiveMode::Recursive).unwrap();
//
//        loop {
//            park();
//            for res in &rx {
//                match &res {
//                    Ok(event) => {
//                        let result = SerializedEvent {
//                            kind: match event.kind {
//
//                            }
//                        };
//                    },
//                    Err(e) => {
//                        app_handle.emit("error", &e)
//                    }
//                };
//            }
//        }
//    });
//}

pub fn init_vault_load(app_handle: tauri::AppHandle, path: &Path) -> Result<Vec<PathBuf>, String> {
    let root = get_default_vault_path(app_handle)?;
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let rel_path = entry.path().strip_prefix(&root).unwrap();
            files.push(rel_path.to_path_buf());
        } else if entry.file_type().is_dir() {
            let rel_path = entry.path().strip_prefix(&root).unwrap();
            files.push(rel_path.to_path_buf());
        }
    }
    return Ok(files);
}

#[tauri::command]
pub fn get_cards(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let vault_path = get_default_vault_path(app_handle)?;
    let mut files = Vec::new();
    let cardpath = vault_path.join("Cards");
    for entry in WalkDir::new(cardpath).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(fs::read_to_string(entry.path()).unwrap());
        }
    }
    return Ok(files);
}
