use crate::{
    db::{
        db_client::db_conn,
        schema::lesson_items::dsl as items,
        schema::user_completed_lessons,
    },
    models::{lesson::Lesson, user_lesson::UserCompletedLesson},
};
use diesel::prelude::*;

pub fn get_lessons(difficulty: String) -> Result<Vec<Lesson>, String> {
    let conn = &mut db_conn();

    items::lesson_items
        .filter(items::is_test.eq(false))
        .filter(items::difficulty.eq(difficulty))
        .order(items::id.asc())
        .select(Lesson::as_select())
        .load::<Lesson>(conn)
        .map_err(|e| format!("Error loading lessons: {:?}", e))
}

pub fn get_lesson_tests(lesson_id: i32) -> Result<Vec<Lesson>, String> {
    let conn = &mut db_conn();

    items::lesson_items
        .filter(items::is_test.eq(true))
        .filter(items::parent_lesson_id.eq(Some(lesson_id)))
        .order(items::id.asc())
        .load::<Lesson>(conn)
        .map_err(|e| format!("Error loading lesson tests: {:?}", e))
}

pub fn mark_lesson_completed(user_id: i32, lesson_id: i32) -> Result<(), String> {
    let conn = &mut db_conn();
    let new_completion = UserCompletedLesson { user_id, lesson_id };

    diesel::insert_into(user_completed_lessons::table)
        .values(&new_completion)
        .on_conflict((
            user_completed_lessons::user_id,
            user_completed_lessons::lesson_id,
        ))
        .do_nothing()
        .execute(conn)
        .map_err(|e| format!("Error marking lesson as completed: {:?}", e))?;

    Ok(())
}

pub fn get_completed_lessons(user_id: i32) -> Result<Vec<i32>, String> {
    let conn = &mut db_conn();

    user_completed_lessons::table
        .filter(user_completed_lessons::user_id.eq(user_id))
        .select(user_completed_lessons::lesson_id)
        .load::<i32>(conn)
        .map_err(|e| format!("Error fetching completed lessons: {:?}", e))
}
