use crate::db::schema::user_completed_lessons;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[diesel(table_name = user_completed_lessons)]
pub struct UserCompletedLesson {
    pub user_id: i32,
    pub lesson_id: i32,
}
