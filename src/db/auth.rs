use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get_id_from_link(link: String, conn: &PgConnection) -> Result<i32,()> {
  
    use crate::schema::activation_links::dsl::*;
    use crate::models::users::ActLink;
    
    let link_vec = activation_links.filter(reference.eq(link))
    .load::<ActLink>(conn)
    .expect("error while loading user reference");

    if let Some(link_data) = link_vec.get(0) {
        diesel::delete(activation_links.filter(id.eq(link_data.id)))
        .execute(conn)
        .expect("Error deleting links in get id from link");
        Ok(link_data.user_id)
    } else {
        print!("error in getting id in get_id_from_link");
        Err(())
    }
}

pub fn activate_user(link: String, conn: &PgConnection ) -> Result<(),()> {
    
    use crate::schema::users::dsl::*;

    let user_id = get_id_from_link(link, conn);
    let user_id = match user_id {
        Ok(u) => u,
        Err(_) => return Err(()),
    };

    diesel::update(users)
    .filter(id.eq(user_id))
    .set(verifyed.eq(true))
    .execute(conn)
    .expect("error in func activate user while updating table users");
    Ok(())

}

pub fn link_exists(link: String, conn: &PgConnection) -> bool {
    
    use crate::schema::activation_links::dsl::*;
    use crate::models::users::ActLink;

    let link_vec = activation_links
    .filter(reference.eq(&link))
    .load::<ActLink>(conn)
    .expect("error while loading user reference");

    print!("generated link: {}\n",link);
    if link_vec.len() == 0 {
        false
    } else {
        true
    }

}