extern crate serde;
use serde::{Serialize, Deserialize};

use crate::schema::priveleges;

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

