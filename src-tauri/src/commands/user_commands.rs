use crate::{models::user::User, services::user_service};

#[tauri::command]
pub fn get_all_users() -> Vec<User> {
    user_service::get_users()
}