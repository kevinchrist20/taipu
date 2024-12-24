use crate::{
    models::{lesson::Lesson, lesson_test::LessonTest},
    services::lesson_service,
};

#[tauri::command]
pub fn get_lessons_by_difficulty(difficulty: String) -> Result<Vec<Lesson>, String> {
    lesson_service::get_lessons(difficulty)
}

#[tauri::command]
pub fn get_lesson_tests(id: i32) -> Result<Vec<LessonTest>, String> {
    lesson_service::get_lesson_test(id)
}
