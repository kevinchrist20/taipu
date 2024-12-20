// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod commands;
mod db;
mod models;
mod services;
mod utils;

use commands::user_commands;
use commands::utils_commands;
use db::db_client;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            db_client::init_db();

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            utils_commands::exit_app,
            user_commands::get_all_users,
            user_commands::add_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
