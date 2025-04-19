use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Queryable, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Lesson {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub difficulty: String,
    pub language: String,
    pub category: String,
    pub is_test: bool,
    pub parent_lesson_id: Option<i32>,
    pub passing_wpm: Option<i32>,
    pub accuracy_threshold: Option<i32>,
    #[ts(type = "string | null")]
    pub created_at: Option<NaiveDateTime>,
    #[ts(type = "string | null")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::lesson_items)]
pub struct NewLesson {
    pub title: String,
    pub difficulty: String,
    pub content: String,
    pub language: Option<String>,
    pub category: String
}
