use chrono::NaiveDateTime;
extern crate serde;
use crate::Error;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    subscribes,
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable, SmallInt,
};
use crate::models::users::Users;

#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct Subscribes {
    pub id: i32,
    pub from_id: i32,
    pub to_id: i32,
}

#[derive(Serialize,Clone,Debug,QueryableByName)]
pub struct User {
    #[sql_type="Integer"]
    pub id: i32,
    #[sql_type="Text"]
    pub username: String,
    #[sql_type="Timestamp"]
    pub register_data: NaiveDateTime,
    #[sql_type="Bool"]
    pub in_subs: bool,
    #[sql_type="Text"]
    pub picture_path: String,
}

impl Subscribes {
    pub fn create(from: i32, to: i32, conn: &PgConnection) -> Result<Subscribes,Error> {

        use crate::schema::subscribes::dsl::*;
        
        let r = diesel::insert_into(subscribes)
        .values(&(from_id.eq(from),to_id.eq(to)))
        .get_result::<Subscribes>(conn)?;
        Ok(r)

    }

    pub fn subscribers(u_id: i32, limit: i32, offset: i32, conn: &PgConnection) -> Result<Vec<User>,Error> {

        let subs = diesel::sql_query(include_str!("../../SQL/subs_list.sql"))
            .bind::<Integer, _>(u_id)
            .bind::<Integer, _>(limit)
            .bind::<Integer, _>(offset)
            .get_results::<User>(conn)?;         
        Ok(subs)
    }

    pub fn delete(sub: i32, user: i32, conn: &PgConnection) -> Result<(),Error> {

        use crate::schema::subscribes::dsl::*;

        diesel::delete(subscribes
            .filter(to_id.eq(user))
            .filter(from_id.eq(sub)))
            .execute(conn)?;
        Ok(())

    }

    pub fn subscribed(u_id: i32, limit: i32, offset: i32, conn: &PgConnection) -> Result<Vec<User>,Error> {

        let subs = diesel::sql_query(include_str!("../../SQL/me_subs_list.sql"))
            .bind::<Integer, _>(u_id)
            .bind::<Integer, _>(limit)
            .bind::<Integer, _>(offset)
            .get_results::<User>(conn)?; 
        Ok(subs)
    }

    pub fn count(u_id: i32, conn: &PgConnection) -> Result<(i64,i64),Error> {

        use crate::schema::subscribes::dsl::*;
        use diesel::dsl::count;

        let you = subscribes
            .filter(from_id.eq(u_id))
            .select(count(id))
            .first::<i64>(conn)?;

        let yours = subscribes
            .filter(to_id.eq(u_id))
            .select(count(id))
            .first::<i64>(conn)?;

        Ok((you,yours))

    }

    pub fn exists(from: i32, to: i32, conn: &PgConnection) -> Result<bool,Error> {

        use crate::schema::subscribes::dsl::*;
        use diesel::dsl::count;

        let c = subscribes
        .filter(to_id.eq(to))
        .filter(from_id.eq(from))
        .select(count(id))
        .first::<i64>(conn)?;

        Ok(c > 0)
    }
}
