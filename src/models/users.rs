extern crate serde;
use serde::{Serialize, Deserialize};
use diesel::*;

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
    pub is_banned: bool,
}

impl Users {

    pub fn get_by_id(u_id: i32, conn: &PgConnection) -> Users {
        
        use crate::schema::users::dsl::*;

        
        users
            .filter(id.eq(u_id))
            .get_result::<Users>(conn)
            .expect("Error getting user by id in get_user_by_id")
    }
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