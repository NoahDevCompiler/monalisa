use notify::{Event, EventKind, Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::str::SplitWhitespace;
use std::sync::mpsc::channel;
use std::thread::{park, spawn};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use walkdir::WalkDir;
use std::sync::{self, Arc, Mutex};
use once_cell::sync::Lazy;

//global vault value for the active or last active vault path
static VAULT_PATH: Lazy<Arc<Mutex<Option<PathBuf>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

//pub fn get_default_vault_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
//    let vault_path = app_handle
//        .path()
//        .resolve("Monalisa Vault", BaseDirectory::Document);
//
//    let vault_path = match vault_path {
//        Ok(path) => path,
//        Err(e) => return Err(format!("Error calling path: {}", e)),
//    };
//
//    if !vault_path.exists() {
//        if let Err(e) = fs::create_dir(&vault_path) {
//            return Err(format!("Couldnt create vault {}", e));
//        }
//    }
//    Ok(vault_path)
//}

#[tauri::command]
fn set_vault_path(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(path);
    let mut vault_path = VAULT_PATH.lock().map_err(|e| e.to_string())?;
    *vault_path = Some(path_buf);
    Ok(())
}



struct SerializedEvent {
    kind: String,
    paths: Vec<PathBuf>,
}

impl From <Event> for SerializedEvent {
    fn from(event: Event) -> Self {
        SerializedEvent { kind: format!("{:?}", event.kind), paths: event.paths.clone()}
    }
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

//pub fn init_vault_load(app_handle: &AppHandle, path: &Path)-> Result<Vec<PathBuf>, String> {
//
//    let files = Vec::new();
//    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
//        if entry.file_type().is_file() {
//            let rel_path = entry.path().strip_prefix(root).unwrap();
//            files.push(rel_path.to_path_buf());
//        }
//        else if entry.file_type().is_dir() {
//            let rel_path = entry.path().strip_prefix(root).unwrap();
//            files.push(rel_path.to_path_buf());
//        }
//    }
//    return Ok(files);
//
//}
