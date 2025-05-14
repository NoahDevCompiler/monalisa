use tauri::AppHandle;
mod config;
mod initial;
mod models;
mod state;
mod utils;
use config::commands::{get_config, set_active_vault};
use config::manager::{load_config, save_config};
use initial::init_app_dirs;
use state::ConfigState;
pub mod errors;
use crate::config::manager::AppConfig;
use crate::utils::fs_manager;
use tauri::Manager;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};

fn init_plugin<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("dummy").build()
}

#[tauri::command]
fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn maximize_window(window: tauri::Window) {
    window.maximize().unwrap();
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    window.close().unwrap();
}
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.handle();
            let config_loader = match load_config(app_handle.clone()) {
                Ok(config) => config,
                Err(_) => {
                    let default = AppConfig::default();
                    save_config(&default, app_handle.clone());
                    default
                }
            };

            app.manage(ConfigState(std::sync::Mutex::new(config_loader)));
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            maximize_window,
            close_window,
            get_config,
            set_active_vault,
            fs_manager::create_folder,
            fs_manager::create_md_file,
            fs_manager::read_directory,
            fs_manager::read_file,
            fs_manager::write_file,
            fs_manager::get_cards,
            fs_manager::create_vault,
            config::commands::get_active_vault_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
