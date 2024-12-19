use crate::{db::db_client::db_conn, db::schema::users::dsl, models::user::User};
use diesel::prelude::*;

pub fn get_users() -> Vec<User> {
    let conn = &mut db_conn();

    dsl::users
        .order_by(dsl::created_at.desc())
        .load::<User>(conn)
        .expect("Error loading users")
}
