use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::priveleges::dsl::*;
use crate::models::admin::{Priveleges,NewPrivelege};

use crate::admin::UserPriveleges;

pub fn create_privelege(uid: i32, priv_type: UserPriveleges, conn: &PgConnection ) -> () {

    let privelege = NewPrivelege {
        user_id: uid,
        privelege_type: priv_type.get_string(),
    };

    diesel::insert_into(priveleges)
        .values(&privelege)
        .execute(conn)
        .expect("Error creating privelege");
}

pub fn update_privelege(uid: i32, conn: &PgConnection) -> () {

    diesel::delete(priveleges.filter(user_id.eq(uid)))
        .execute(conn)
        .expect("Got an error while deleting privelege");

}

pub fn change_privelege(uid: i32, new_type: UserPriveleges, conn: &PgConnection) -> () {

    diesel::update(priveleges)
        .filter(user_id.eq(uid))
        .set( privelege_type.eq(new_type.get_string()))
        .execute(conn);

}

pub fn get_priveleges(uid: i32, conn: &PgConnection) -> UserPriveleges {

    let db_result = priveleges
        .filter(user_id.eq(uid))
        .load::<Priveleges>(conn)
        .expect("Error in finding priveleges (may not exist)");

    if let Some(privelege) = db_result.get(0) {
        UserPriveleges::set_by_string(privelege.privelege_type.clone())
            .expect("mismatch while creating UserPrivelege from string")
    } else {
        panic!("no priveleges found for this user")
    }
  
}


