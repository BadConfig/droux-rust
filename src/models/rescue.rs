use chrono::NaiveDateTime;
extern crate serde;
use crate::Error;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    rescue_issues,
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable, SmallInt,
};
use crate::models::users::Users;

#[derive(Serialize,Clone,Debug,Queryable)]
pub struct Issue {
    id: i32,
    contact: String,
    issue: String,
}

impl Issue {

    pub fn create(addr: &String, description: &String, conn: &PgConnection) -> Result<(),Error> {
        use crate::schema::rescue_issues::dsl::*;

        diesel::insert_into(rescue_issues)
            .values(&(contact.eq(addr),issue.eq(description)))
            .execute(conn)?;
        Ok(())
    }
    pub fn delete(issue_id: i32, conn: &PgConnection) -> Result<(),Error> {
        use crate::schema::rescue_issues::dsl::*;

        diesel::delete(rescue_issues
            .filter(id.eq(issue_id)))
            .execute(conn)?;
        Ok(())
    }
    pub fn list(offset: i64, limit: i64, conn: &PgConnection) -> Result<Vec<Issue>,Error> {
        use crate::schema::rescue_issues::dsl::*;

        Ok(rescue_issues
            .limit(limit)
            .offset(offset)
            .get_results::<Issue>(conn)?)
    }
}
