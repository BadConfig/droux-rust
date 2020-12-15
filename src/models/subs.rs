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

#[derive(Serialize,Clone,Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub register_data: NaiveDateTime,
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

    pub fn subscribers(u_id: i32, limit: i64, offset: i64, conn: &PgConnection) -> Result<Vec<User>,Error> {

        use crate::schema::subscribes::dsl::*;
        use crate::schema::users;
        use crate::models::users::Users;

        let subs = subscribes
            .inner_join(
                users::table.on(
                    users::id.eq(from_id))
            )
            .select((
                users::id,
                users::username,
                users::register_data,
                users::picture_path,
            ))
            .filter(to_id.eq(u_id))
            .limit(limit)
            .offset(offset)
            .get_results::<(i32,String,NaiveDateTime,String)>(conn)?;
        let r: Vec<User> = subs
            .into_iter()
            .map( |s|
            User {
                id: s.0,
                username: s.1,
                register_data: s.2,
                picture_path: s.3,
            })
            .collect();
        Ok(r)
    }

    pub fn delete(sub: i32, user: i32, conn: &PgConnection) -> Result<(),Error> {

        use crate::schema::subscribes::dsl::*;

        diesel::delete(subscribes
            .filter(to_id.eq(user))
            .filter(from_id.eq(sub)))
            .execute(conn)?;
        Ok(())

    }

    pub fn subscribed(u_id: i32, limit: i64, offset: i64, conn: &PgConnection) -> Result<Vec<User>,Error> {

        use crate::schema::subscribes::dsl::*;
        use crate::schema::users;
        use crate::models::users::Users;

        let subs = subscribes
            .inner_join(
                users::table.on(
                    users::id.eq(from_id))
            )
            .select((
                users::id,
                users::username,
                users::register_data,
                users::picture_path,
            ))
            .filter(from_id.eq(u_id))
            .limit(limit)
            .offset(offset)
            .get_results::<(i32,String,NaiveDateTime,String)>(conn)?;
        let r: Vec<User> = subs
            .into_iter()
            .map( |s|
            User {
                id: s.0,
                username: s.1,
                register_data: s.2,
                picture_path: s.3,
            })
            .collect();
        Ok(r)
    }

    pub fn count(u_id: i32, conn: &PgConnection) -> Result<(i64,i64),Error> {

        use crate::schema::subscribes::dsl::*;
        use diesel::dsl::count;

        let you = subscribes
            .filter(to_id.eq(u_id))
            .select(count(id))
            .first::<i64>(conn)?;

        let yours = subscribes
            .filter(from_id.eq(u_id))
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
