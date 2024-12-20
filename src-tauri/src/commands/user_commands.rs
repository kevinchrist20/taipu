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
