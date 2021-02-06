use chrono::NaiveDateTime;
extern crate serde;
use crate::Error;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    promos
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable, SmallInt,
};
use crate::models::users::Users;

#[derive(Serialize,Clone,Debug,Queryable)]
pub struct Promo {
    id: i32,
    promo: i32,
    sale: i32,
}

impl Promo {
    pub fn get_sale(value: f64, try_promo: String, conn: &PgConnection) -> f64 {
        use crate::schema::promos::dsl::*;
        let my_sale = promos
            .filter(promo.eq(try_promo))
            .select(sale)
            .get_result::<i32>(conn).unwrap_or(100) as f64;
        value/100.0*my_sale
    }
}