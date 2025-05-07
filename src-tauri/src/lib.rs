#![allow(dead_code)]
mod config;
mod initial;
mod utils;
mod models;
use initial::init_app_dirs;
mod vault;
pub mod errors;
//pub mod state;
use crate::utils::fs_manager;


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
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            maximize_window,
            close_window,
            fs_manager::create_folder,
            fs_manager::create_md_file,
            fs_manager::read_directory,
            fs_manager::read_file,
<<<<<<< HEAD
            fs_manager::write_file,
            fs_manager::get_cards
=======
            fs_manager::write_file
>>>>>>> a1c51f146ddd533656bc92b4961c423d8e7b0598
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
