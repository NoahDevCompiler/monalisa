mod initial;
mod utils;
mod config;
use initial::init_app_dirs;
use utils::fs_manager;
mod vault;
use vault::manager;
use vault::fs_operations;
pub mod state;
pub mod errors;

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
        .setup(|app| {
            init_app_dirs::init_default(app.handle())?;
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            maximize_window,
            close_window,
            //fs_manager::create_folder,
            //fs_manager::create_md_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
