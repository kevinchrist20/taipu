use crate::{
    db::{
        db_client::db_conn,
        schema::{self, lessons::dsl},
    },
    models::{lesson::Lesson, lesson_test::LessonTest},
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
