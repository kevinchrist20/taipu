use std::collections::HashMap;

use crate::{
    db::{
        db_client::db_conn,
        schema::{lesson_items::dsl as items, user_completed_lessons},
    },
    models::{
        lesson::{CategoryWithLessons, Lesson},
        user_lesson::UserCompletedLesson,
    },
};
use diesel::prelude::*;

pub fn get_lessons(difficulty: String) -> Result<Vec<Lesson>, String> {
    let conn = &mut db_conn();

    items::lesson_items
        .filter(items::is_test.eq(false))
        .filter(items::difficulty.eq(difficulty))
        .order(items::id.asc())
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

pub fn get_category_tests(category: &str, difficulty: &str) -> Result<Vec<Lesson>, String> {
    let conn = &mut db_conn();

    items::lesson_items
        .filter(items::is_test.eq(true))
        .filter(items::category.eq(category))
        .filter(items::difficulty.eq(difficulty))
        .order(items::id.asc())
        .load::<Lesson>(conn)
        .map_err(|e| format!("Error loading category tests: {:?}", e))
}

pub fn get_lessons_by_categories(
    difficulty: String,
    user_id: i32,
) -> Result<Vec<CategoryWithLessons>, String> {
    let conn = &mut db_conn();

    let all_lessons: Vec<Lesson> = items::lesson_items
        .filter(items::is_test.eq(false))
        .filter(items::difficulty.eq(&difficulty))
        .order_by(items::id.asc())
        .load::<Lesson>(conn)
        .map_err(|e| format!("Error loading lessons: {:?}", e))?;

    let completed_lessons = get_completed_lessons(user_id)?;
    let completed_lessons_set: std::collections::HashSet<i32> =
        completed_lessons.into_iter().collect();

    let mut categories_map: HashMap<String, Vec<Lesson>> = HashMap::new();
    for lesson in all_lessons {
        let category_name = lesson.category.clone();
        categories_map
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(lesson);
    }

    let category_order = vec![
        "home-left",
        "home-right",
        "home-combined",
        "top-row",
        "bottom-row",
        "row-transitions",
        "punctuation",
        "common-words",
        "numbers-row",
        "speed-drills",
    ];

    // Sort categories by the predefined order
    let mut sorted_categories: Vec<String> = Vec::new();
    for category in &category_order {
        if categories_map.contains_key(*category) {
            sorted_categories.push(category.to_string());
        }
    }

    let mut result = Vec::new();
    let mut is_previous_category_completed = true; // First category is always available

    for category_name in sorted_categories {
        let mut lessons = categories_map.remove(&category_name).unwrap_or_default();

        lessons.sort_by_key(|lesson| lesson.id);
        let tests = get_category_tests(&category_name, &difficulty)?;

        // A category is available if the previous category is completed
        let is_available = is_previous_category_completed;

        // Consider this category completed if all its lessons and at least one of its tests are completed
        let is_category_completed = if !is_available {
            false
        } else {
            let all_lessons_completed = lessons
                .iter()
                .all(|lesson| completed_lessons_set.contains(&lesson.id));

            let any_test_completed = tests
                .iter()
                .any(|test| completed_lessons_set.contains(&test.id));

            all_lessons_completed && (tests.is_empty() || any_test_completed)
        };

        result.push(CategoryWithLessons {
            category: category_name,
            lessons,
            tests,
            is_available,
        });

        is_previous_category_completed = is_category_completed;
    }

    Ok(result)
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

pub fn get_lesson_by_id(lesson_id: i32) -> Result<Lesson, String> {
    let conn = &mut db_conn();

    items::lesson_items
        .find(lesson_id)
        .select(Lesson::as_select())
        .first::<Lesson>(conn)
        .map_err(|e| format!("Error occurred while getting lesson: {:?}", e))
}
