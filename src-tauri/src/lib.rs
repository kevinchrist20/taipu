// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod commands;

use commands::utils_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            utils_commands::greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
