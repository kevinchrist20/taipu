use crate::{models::user::{NewUser, User}, services::user_service};

#[tauri::command]
pub fn get_all_users() -> Vec<User> {
    user_service::get_users()
}

#[tauri::command]
pub fn add_user(new_user: NewUser) {
    user_service::create_user(&new_user)
}