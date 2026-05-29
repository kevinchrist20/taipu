use crate::{models::stats::UserStatistics, services::stats_service};

#[tauri::command]
pub fn get_user_statistics(user_id: i32, difficulty: String) -> Result<UserStatistics, String> {
    stats_service::get_user_statistics(user_id, difficulty)
}
