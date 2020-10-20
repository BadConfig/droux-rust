extern crate serde;
use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub pass_hash: String,
    pub picture_path: String,
    pub verifyed: bool,
    pub rate_summ: i64,
    pub rate_count: i64,
    pub register_data: chrono::NaiveDateTime,
}

#[derive(Insertable,AsChangeset,Debug)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub pass_hash: String,
}

use crate::schema::activation_links;

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ActLink {
    pub id: i32,
    pub user_id: i32,
    pub reference: String,
}

#[derive(Insertable,AsChangeset)]
#[table_name="activation_links"]
pub struct NewActLink {
    pub user_id: i32,
    pub reference: String,
}