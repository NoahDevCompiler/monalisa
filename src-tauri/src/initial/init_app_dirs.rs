use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use std::fs;
use tauri_plugin_log::Error;
use crate::state::SharedState;

pub fn init_default(app_handle: &AppHandle) -> Result<(), String> {
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

    let app_config_dir = match app_handle
    .path()
    .resolve(".monalisa", BaseDirectory::Config) {
        Ok(path) => path,
        Err(ad_error) => {
            println!("Appdata failed");
            match app_handle.path().resolve(".monalisa", BaseDirectory::Temp) {
                Ok(path) => path,
                Err(temp_err) => {
                    println!("Temp and Appdata failed");
                    return Err(format!("Failed creating config Folder {}, {}", temp_err, ad_error))
                }
            }
        }
    };

    if let Err(e) = fs::create_dir_all(&app_config_dir) {
        println!("Creation error");
        return Err(format!("Couldnt create App Config {}", e));
    }
    println!("Successfully created {}", app_config_dir.display());
    Ok(())

}

