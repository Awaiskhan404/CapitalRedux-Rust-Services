use crate::db::PgPool;
use crate::models::{NewUser, User};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_user(conn: &PgPool, new_user: NewUser) -> Result<User, Error> {
    conn.transaction(|| {
        diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .get_result(conn)
    })
}