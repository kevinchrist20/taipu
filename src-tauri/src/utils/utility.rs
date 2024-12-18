use crate::models::user::{KeyboardType, Language, LessonDifficulty, Theme};

pub fn to_keyboard_type(keyboard_type: &str) -> KeyboardType {
    match keyboard_type {
        "QWERTY" => KeyboardType::Qwerty,
        "AZERTY" => KeyboardType::Azerty,
        _ => KeyboardType::Qwerty,
    }
}

pub fn to_language(language: &str) -> Language {
    match language {
        "English" => Language::English,
        "French" => Language::French,
        _ => Language::English,
    }
}

pub fn to_theme(theme: &str) -> Theme {
    match theme {
        "light" => Theme::Light,
        "dark" => Theme::Dark,
        _ => Theme::Light,
    }
}

pub fn to_difficulty(difficulty: &str) -> LessonDifficulty {
    match difficulty {
        "beginner" => LessonDifficulty::Beginner,
        "intermediate" => LessonDifficulty::Intermediate,
        "advanced" => LessonDifficulty::Advanced,
        _ => LessonDifficulty::Beginner,
    }
}

pub fn to_string<T: ToString>(enum_value: &T) -> String {
    enum_value.to_string()
}