use chrono::NaiveDateTime;
extern crate serde;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    products,
    categories,
    sub_categories,
    favourites,
    promotions,
    rating,
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable, SmallInt,
};
use rocket_contrib::templates::tera::*;
use crate::models::users::Users;

