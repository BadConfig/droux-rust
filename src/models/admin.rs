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

#[derive(Serialize, Deserialize)]
pub struct PrivelegesUsers {
    id: i32,
    username: String,
    image: String,
    privelege: String,
}

impl Priveleges {
    pub fn get(conn: &PgConnection) -> Result<Vec<PrivelegesUsers>,Error> {
        use crate::schema::users;

        let res = priveleges::table
            .inner_join(users::table.on(users::id.eq(priveleges::user_id)))
            .select((users::id,users::username,users::picture_path,priveleges::privelege_type))
            .get_results::<(i32,String,String,String)>(conn)?;
        Ok(res
            .into_iter()
            .map(| x | { PrivelegesUsers { id: x.0, username: x.1, image: x.2, privelege: x.3 } })
            .collect())
    }

    pub fn change(u_id: i32, pr_type: String, conn: &PgConnection) -> Result<(),Error> {

        diesel::update(priveleges::table
            .filter(priveleges::user_id.eq(u_id)))
            .set(priveleges::privelege_type.eq(pr_type))
            .execute(conn)?;

        Ok(())
    }

    pub fn insert(u_name: String, pr_type: String, conn: &PgConnection) -> Result<(),Error> {

        use crate::schema::users;

        let u_id = users::table
            .filter(users::username.eq(u_name))
            .select(users::id)
            .get_result::<i32>(conn)?;
        
        diesel::insert_into(priveleges::table)
            .values(
                &(priveleges::user_id.eq(u_id),
                priveleges::privelege_type.eq(pr_type))
            )
            .execute(conn)?;

        Ok(())
    }

    pub fn delete(u_id: i32, conn: &PgConnection) -> Result<(),Error> {

        diesel::delete(priveleges::table
            .filter(priveleges::user_id.eq(u_id)))
            .execute(conn)?;

        Ok(())
    }

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
