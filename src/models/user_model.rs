use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Nullable, Text};
use serde::{Deserialize, Serialize};

use crate::schema::{extended_user, users};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Queryable)]
pub struct ExtendedUser {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_entrepreneur: bool,
    pub is_admin: bool,
    pub is_investor: bool,
    pub user_type: String,
    pub cash: i32,
    pub net_assets: i32,
    pub net_liabilities: i32,
    pub net_worth: i32,
    pub profile_pic: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Insertable)]
#[table_name = "extended_user"]
pub struct NewExtendedUser<'a> {
    pub user_id: i32,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub is_entrepreneur: bool,
    pub is_admin: bool,
    pub is_investor: bool,
    pub user_type: &'a str,
    pub cash: i32,
    pub net_assets: i32,
    pub net_liabilities: i32,
    pub net_worth: i32,
    pub profile_pic: Option<&'a str>,
    pub bio: Option<&'a str>,
    pub location: Option<&'a str>,
    pub website: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub facebook: Option<&'a str>,
    pub twitter: Option<&'a str>,
    pub instagram: Option<&'a str>,
    pub linkedin: Option<&'a str>,
}