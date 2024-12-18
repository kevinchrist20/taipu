use crate::models::user::{KeyboardType, Language, LessonDifficulty, Theme};

pub fn to_keyboard_type(keyboard_type: &str) -> KeyboardType {
    match keyboard_type.to_lowercase().as_str() {
        "qwerty" => KeyboardType::Qwerty,
        "azerty" => KeyboardType::Azerty,
        _ => KeyboardType::Qwerty,
    }
}

pub fn to_language(language: &str) -> Language {
    match language.to_lowercase().as_str() {
        "english" => Language::English,
        "french" => Language::French,
        _ => Language::English,
    }
}

pub fn to_theme(theme: &str) -> Theme {
    match theme.to_lowercase().as_str() {
        "light" => Theme::Light,
        "dark" => Theme::Dark,
        _ => Theme::Light,
    }
}

pub fn to_difficulty(difficulty: &str) -> LessonDifficulty {
    match difficulty.to_lowercase().as_str() {
        "beginner" => LessonDifficulty::Beginner,
        "intermediate" => LessonDifficulty::Intermediate,
        "advanced" => LessonDifficulty::Advanced,
        _ => LessonDifficulty::Beginner,
    }
}