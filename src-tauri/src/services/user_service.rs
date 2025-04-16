use crate::{
    db::{db_client::db_conn, schema::users, schema::users::dsl},
    models::user::{NewUser, User},
};
use diesel::prelude::*;

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
