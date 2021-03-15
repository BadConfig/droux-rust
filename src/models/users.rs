extern crate serde;
use serde::{Serialize, Deserialize};
use diesel::*;

use crate::schema::users;
use crate::Error;

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

    pub fn set_photo(self, filename: String, conn: &PgConnection) -> Result<(),Error> {
        use crate::schema::users::dsl::*;
        
        diesel::update(users.filter(id.eq(self.id)))
            .set(picture_path.eq(filename))
            .execute(conn)?;
        Ok(())
    }
    pub fn get_by_id(u_id: i32, conn: &PgConnection) -> Users {
        
        use crate::schema::users::dsl::*;

        
        users
            .filter(id.eq(u_id))
            .get_result::<Users>(conn)
            .expect("Error getting user by id in get_user_by_id")
    }

    pub fn get_with_page(page: i64, conn: &PgConnection) ->  Result<Vec<Users>,Error> {
        
        use crate::schema::users::dsl::*;

        Ok(users
            .limit(18)
            .offset(18*(page-1))
            .get_results::<Users>(conn)?)

    }

    pub fn change_banned(flag: bool, u_id: i32, conn: &PgConnection) -> Result<(),Error> {
      
        use crate::schema::users::dsl::*;

        diesel::update(users)
            .filter(id.eq(u_id))
            .set(is_banned.eq(flag))
            .execute(conn)?;
        Ok(())
        
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
