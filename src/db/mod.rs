pub mod admin;
pub mod filters;
pub mod auth;
pub mod product;
pub mod users;
pub mod chat;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use rocket_contrib::databases::diesel;
use rocket_contrib::database;

#[database("diesel_db")]
pub struct Conn(diesel::PgConnection);


pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}