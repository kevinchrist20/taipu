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
    pub avatar: String,
    pub language: String,
    pub lesson_difficulty: String,
    #[ts(type = "string | null")]
    pub created_at: Option<NaiveDateTime>,
    #[ts(type = "string | null")]
    pub last_active: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
    pub name: String,
    pub avatar: String,
    pub language: String,
    pub lesson_difficulty: String,
}
