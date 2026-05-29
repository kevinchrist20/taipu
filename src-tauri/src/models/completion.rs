use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::completions)]
pub struct NewCompletion {
    pub user_id: i32,
    pub lesson_id: i32,
    pub wpm: f64,
    pub accuracy: f64,
    pub grade: String,
    pub duration_seconds: i32,
}
