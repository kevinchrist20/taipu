use crate::{db::{db_client::db_conn, schema::users::dsl, schema::users}, models::user::{NewUser, User}};
use diesel::prelude::*;

pub fn get_users() -> Vec<User> {
    let conn = &mut db_conn();

    dsl::users
        .order_by(dsl::created_at.desc())
        .load::<User>(conn)
        .expect("Error loading users")
}

pub fn create_user(new_user: &NewUser) {
    let conn = &mut db_conn();

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .expect("Error saving new user");
}
