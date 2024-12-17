use chrono::{serde::ts_seconds_option, DateTime, Utc};
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, TS)]
#[diesel(table_name = crate::db::schema::users)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    #[serde(with = "ts_seconds_option")]
    #[ts(type = "string | null")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    #[ts(type = "string | null")]
    pub last_active: Option<DateTime<Utc>>,
    pub keyboard_type: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub lesson_difficulty: Option<String>,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum KeyboardType {
    Qwerty,
    Azerty,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Language {
    English,
    French,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum LessonDifficulty {
    Beginner, 
    Intermediate, 
    Advanced
}
