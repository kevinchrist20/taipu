use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use ts_rs::TS;


#[derive(Queryable, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    #[ts(type = "string | null")]
    pub created_at: Option<NaiveDateTime>,
    #[ts(type = "string | null")]
    pub last_active: Option<NaiveDateTime>,
    pub keyboard_type: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub lesson_difficulty: Option<String>,
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
