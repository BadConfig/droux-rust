extern crate serde;
use serde::{Serialize, Deserialize};

use crate::schema::priveleges;
use diesel::PgConnection;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Priveleges {
    pub id: i32,
    pub user_id: i32,
    pub privelege_type: String,
}

#[derive(Insertable,AsChangeset)]
#[table_name="priveleges"]
pub struct NewPrivelege {
    pub user_id: i32,
    pub privelege_type: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Links {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub icon: String,
}

impl Links {

    pub fn get_social_media_links(conn: &PgConnection) -> Vec<Links> {
        
        crate::schema::social_links::table
            .load::<Links>(conn)
            .expect("Error loading links")

    }

}
