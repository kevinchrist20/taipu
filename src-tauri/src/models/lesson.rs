use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use ts_rs::TS;


#[derive(Insertable, Queryable, Debug, Serialize,  Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[diesel(table_name = crate::db::schema::lessons)]
pub struct Lesson {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub difficulty: String,
    pub language: String,
    #[ts(type = "string | null")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string | null")]
    pub updated_at: NaiveDateTime,
}
