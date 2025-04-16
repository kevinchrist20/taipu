use crate::{
    db::{
        db_client::db_conn,
        schema::{self, lessons::dsl, user_completed_lessons},
    },
    models::{lesson::Lesson, lesson_test::LessonTest, user_lesson::UserCompletedLesson},
};
use diesel::prelude::*;

pub fn get_lessons(difficulty: String) -> Result<Vec<Lesson>, String> {
    let conn = &mut db_conn();

    match dsl::lessons
        .filter(dsl::difficulty.eq(difficulty))
        .order(dsl::id.asc())
        .load::<Lesson>(conn)
    {
        Ok(lessons) => Ok(lessons),
        Err(e) => Err(format!("Error loading lessons: {:?}", e)),
    }
}

pub fn get_lesson_test(lesson_id: i32) -> Result<Vec<LessonTest>, String> {
    let conn = &mut db_conn();

    match schema::tests::dsl::tests
        .filter(schema::tests::dsl::lesson_id.eq(lesson_id))
        .order(schema::tests::dsl::id.asc())
        .load::<LessonTest>(conn)
    {
        Ok(lesson_tests) => Ok(lesson_tests),
        Err(e) => Err(format!("Error loading lesson tests: {:?}", e)),
    }
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
