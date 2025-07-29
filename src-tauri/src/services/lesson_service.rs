use std::{collections::HashMap, fmt::Debug};

use crate::{
    db::{
        db_client::db_conn,
        schema::{lesson_items::dsl as items, user_completed_lessons},
    },
    models::{lesson::{CategoryWithLessons, Lesson}, user_lesson::UserCompletedLesson},
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
        .filter(items::parent_lesson_id.is_null()) // Category tests have null parent_lesson_id
        .order(items::id.asc())
        .load::<Lesson>(conn)
        .map_err(|e| format!("Error loading category tests: {:?}", e))
}

pub fn get_lessons_by_categories(difficulty: String, user_id: i32) -> Result<Vec<CategoryWithLessons>, String> {
    let conn = &mut db_conn();
    
    // Fetch all lessons for the given difficulty
    let all_lessons: Vec<Lesson> = items::lesson_items
        .filter(items::is_test.eq(false))
        .filter(items::difficulty.eq(&difficulty))
        .order_by(items::id.desc())
        .load::<Lesson>(conn)
        .map_err(|e| format!("Error loading lessons: {:?}", e))?;
    
    // Get completed lessons for the user
    let completed_lessons = get_completed_lessons(user_id)?;
    let completed_lessons_set: std::collections::HashSet<i32> = completed_lessons.into_iter().collect();
    
    // Group lessons by category
    let mut categories_map: HashMap<String, Vec<Lesson>> = HashMap::new();
    for lesson in all_lessons {
        let category_name = lesson.category.clone(); // (|| "Uncategorized".to_string())
        categories_map
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(lesson);
    }

    // println!("{:?}", categories_map.fmt());
    
    // Sort categories by order (you may need to add an order field to your schema)
    let mut sorted_categories: Vec<String> = categories_map.keys().cloned().collect();
    // sorted_categories.sort(); // You might want a custom sorting here
    
    // For each category, determine if it's available based on previous category completion
    let mut result = Vec::new();
    let mut is_previous_category_completed = true; // First category is always available
    
    for category_name in sorted_categories {
        let lessons = categories_map.remove(&category_name).unwrap_or_default();
        
        // Get tests for this category
        let tests = get_category_tests(&category_name, &difficulty)?;
        
        // A category is available if the previous category is completed
        let is_available = is_previous_category_completed;
        
        // Consider this category completed if all its lessons and at least one of its tests are completed
        let is_category_completed = if !is_available {
            false
        } else {
            let all_lessons_completed = lessons.iter()
                .all(|lesson| completed_lessons_set.contains(&lesson.id));
                
            let any_test_completed = tests.iter()
                .any(|test| completed_lessons_set.contains(&test.id));
                
            all_lessons_completed && (tests.is_empty() || any_test_completed)
        };
        
        result.push(CategoryWithLessons {
            category: category_name,
            lessons,
            tests,
            is_available,
        });
        
        // Update for next iteration
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
