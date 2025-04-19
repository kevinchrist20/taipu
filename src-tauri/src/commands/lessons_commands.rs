use crate::{
    models::lesson::Lesson,
    services::lesson_service,
};

#[tauri::command]
pub fn get_lessons_by_difficulty(difficulty: String) -> Result<Vec<Lesson>, String> {
    lesson_service::get_lessons(difficulty)
}

#[tauri::command]
pub fn get_lesson_tests(id: i32) -> Result<Vec<Lesson>, String> {
    lesson_service::get_lesson_tests(id)
}

#[tauri::command]
pub fn complete_lesson(user_id: i32, lesson_id: i32) -> Result<(), String> {
    lesson_service::mark_lesson_completed(user_id, lesson_id)
}

#[tauri::command]
pub fn get_completed_lessons(user_id: i32) -> Result<Vec<i32>, String> {
    lesson_service::get_completed_lessons(user_id)
}
