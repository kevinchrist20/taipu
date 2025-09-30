// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod commands;
mod db;
mod models;
mod services;
mod utils;

use commands::lessons_commands;
use commands::user_commands;
use commands::utils_commands;
use db::db_client;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            db_client::init_db();

            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "macos")]
            window
                .eval(
                    r#"
                window.addEventListener('keydown', (e) => {
                    const isContentEditable = document.activeElement.isContentEditable;
                    if (e.key === 'Backspace' && 
                        !['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName) &&
                        !isContentEditable) {
                        e.preventDefault();
                    }
                });
            "#,
                )
                .unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            utils_commands::exit_app,
            user_commands::get_all_users,
            user_commands::add_user,
            lessons_commands::get_lessons_by_difficulty,
            lessons_commands::get_lesson_tests,
            lessons_commands::complete_lesson,
            lessons_commands::get_completed_lessons,
            lessons_commands::get_lessons_by_categories,
            lessons_commands::get_category_tests,
            lessons_commands::get_lesson_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
