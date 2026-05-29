use crate::{
    models::user::{NewUser, User},
    services::user_service,
};

#[tauri::command]
pub fn get_all_users() -> Result<Vec<User>, String> {
    user_service::get_users()
}

#[tauri::command]
pub fn add_user(body: NewUser) -> Result<(), String> {
    user_service::create_user(&body)
}

#[tauri::command]
pub fn update_user_preferences(
    user_id: i32,
    language: String,
    lesson_difficulty: String,
) -> Result<User, String> {
    user_service::update_user_preferences(user_id, &language, &lesson_difficulty)
}
