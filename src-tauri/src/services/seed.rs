use crate::{db::{db_client::db_conn, schema::lessons}, models::{lesson::NewLesson, lesson_test::NewTest}};
use diesel::{dsl, prelude::*};

pub fn seed_database() -> Result<(), diesel::result::Error> {
    let conn = &mut db_conn();
    
    let home_row_lessons = vec![
        NewLesson {
            title: "Home Row: Left Hand (ASDF)".to_string(),
            difficulty: "BEGINNER".to_string(),
            content: "as df sa ad fs ds af sd da fa sf fd as ds af sa df ad fs sa df af sd ds as fa sf ad fs da af ds sf as fd sa df af sd ad".to_string(),
            language: Some("ENGLISH".to_string()),
        },         
        NewLesson {
            title: "Home Row: Left Hand (ASDF) 2".to_string(),
            difficulty: "BEGINNER".to_string(),
            content: "asd dsa fas sdf afd fds sad das fsa ads dfs asf sfd afd dfa sad fas asd sdf fds afd das fsa dfs asf dsa afd sdf fas sad".to_string(),
            language: Some("ENGLISH".to_string()), 
        },        
        NewLesson {
            title: "Home Row: Left Hand (ASDF) 3".to_string(),
            difficulty: "BEGINNER".to_string(),
            content: "asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa".to_string(),
            language: Some("ENGLISH".to_string()),
        },
        NewLesson {
            title: "Home Row: Left Hand (ASDF) 5".to_string(),
            difficulty: "BEGINNER".to_string(),
            content: "dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa fdsad fdsad fdsad fdsad fdsad fdsad fdsad fdsad fdsad".to_string(),
            language: Some("ENGLISH".to_string()),
        }
    ];

    // Insert lessons
    diesel::insert_into(lessons::table)
        .values(&home_row_lessons)
        .execute(conn)?;

    // Insert lessons and fetch IDs
    let lesson_ids: Vec<i32> = lessons::table
        .select(lessons::id)
        .order(lessons::id.asc())
        .limit(home_row_lessons.len() as i64)
        .load(conn)?;

    let tests = vec![
        NewTest {
            lesson_id: *lesson_ids.last().unwrap(),
            title: "Home Row Left Hand Proficiency".to_string(),
            content: "sad dad fast fads ads a sad dad adds a fad a sad dad adds a fad ads and fads add sass a fast sad dad adds sad fads fast ads fade as dad adds a fad".to_string(),
            passing_wpm: 15,
            accuracy_threshold: 85,
        },
    ];

    // Insert tests
    dsl::insert_into(crate::db::schema::tests::table)
        .values(&tests)
        .execute(conn)?;

    Ok(())
}