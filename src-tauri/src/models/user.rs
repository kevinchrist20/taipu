use chrono::{DateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::utils::utility::{to_difficulty, to_keyboard_type, to_language, to_theme};

#[derive(Queryable, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    #[ts(type = "string | null")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    #[ts(type = "string | null")]
    pub last_active: Option<DateTime<Utc>>,
    pub keyboard_type: Option<KeyboardType>,
    pub language: Option<Language>,
    pub theme: Option<Theme>,
    pub lesson_difficulty: Option<LessonDifficulty>,
}

#[derive(Serialize, Deserialize, TS, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum KeyboardType {
    Qwerty,
    Azerty,
}

impl ToString for KeyboardType {
    fn to_string(&self) -> String {
        match self {
            KeyboardType::Qwerty => String::from("qwerty"),
            KeyboardType::Azerty => String::from("azerty"),
        }
    }
}

#[derive(Serialize, Deserialize, TS, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Language {
    English,
    French,
}

#[derive(Serialize, Deserialize, TS, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, TS, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum LessonDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Insertable, Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub keyboard_type: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub lesson_difficulty: Option<String>,
}

impl From<NewUser> for User {
    fn from(new_user: NewUser) -> Self {
        User {
            id: new_user.id,
            name: new_user.name,
            username: new_user.username,
            created_at: None,
            last_active: None,
            keyboard_type: Some(to_keyboard_type(&new_user.keyboard_type.unwrap())),
            language: Some(to_language(&new_user.language.unwrap())),
            theme: Some(to_theme(&new_user.theme.unwrap())),
            lesson_difficulty: Some(to_difficulty(&new_user.lesson_difficulty.unwrap())),
        }
    }
}
