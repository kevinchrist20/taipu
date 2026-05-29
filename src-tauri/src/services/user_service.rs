use crate::{
    db::{db_client::db_conn, schema::users, schema::users::dsl},
    models::user::{NewUser, User},
};
use diesel::prelude::*;

fn normalize_upper(value: &str) -> String {
    value.trim().to_uppercase()
}

fn is_valid_language(language: &str) -> bool {
    matches!(language, "ENGLISH" | "FRENCH")
}

fn is_valid_difficulty(difficulty: &str) -> bool {
    matches!(difficulty, "BEGINNER" | "INTERMEDIATE" | "ADVANCED")
}

pub fn get_users() -> Result<Vec<User>, String> {
    let conn = &mut db_conn();

    match dsl::users
        .order_by(dsl::created_at.desc())
        .load::<User>(conn)
    {
        Ok(users) => Ok(users),
        Err(e) => Err(format!("Error loading users: {:?}", e)),
    }
}

pub fn create_user(new_user: &NewUser) -> Result<(), String> {
    let conn = &mut db_conn();

    match diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) = e
            {
                Err("A user with this username already exists.".to_string())
            } else {
                Err(format!("Error saving new user: {:?}", e))
            }
        }
    }
}

pub fn update_user_preferences(
    user_id: i32,
    language: &str,
    lesson_difficulty: &str,
) -> Result<User, String> {
    let conn = &mut db_conn();

    let language = normalize_upper(language);
    let lesson_difficulty = normalize_upper(lesson_difficulty);

    if !is_valid_language(&language) {
        return Err("Unsupported language value.".to_string());
    }

    if !is_valid_difficulty(&lesson_difficulty) {
        return Err("Unsupported lesson difficulty value.".to_string());
    }

    diesel::update(dsl::users.filter(dsl::id.eq(user_id)))
        .set((
            dsl::language.eq(language),
            dsl::lesson_difficulty.eq(lesson_difficulty),
        ))
        .execute(conn)
        .map_err(|e| format!("Error updating user preferences: {:?}", e))?;

    dsl::users
        .filter(dsl::id.eq(user_id))
        .first::<User>(conn)
        .map_err(|e| format!("Error loading updated user: {:?}", e))
}
