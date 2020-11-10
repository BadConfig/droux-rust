use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::users::dsl::*;
use crate::models::users::{Users,NewUser};

pub fn login_exists(u_login: String, conn: &PgConnection) -> bool {
    let same_users = users.filter(username.eq(u_login))
    .limit(5)
    .load::<Users>(conn)
    .expect("Error loading users from table");

    same_users.len() > 0
}

pub fn email_exists(u_email: String, conn: &PgConnection) -> bool {
    let same_users = users.filter(email.eq(u_email))
    .limit(5)
    .load::<Users>(conn)
    .expect("Error loading users from table");

    same_users.len() > 0
}

pub fn create_user(u_login: String, u_email: String, u_password: String, conn: &PgConnection) -> Result<String,String> {

    use crate::schema::activation_links::dsl::*;
    use crate::models::users::{NewActLink,ActLink};

    if login_exists(u_login.clone(), conn) {
        print!("login error in create user");
        return Err("login".to_string());
    } else if email_exists(u_email.clone(), conn) {
        print!("email error in create user");
        return Err("email".to_string());
    }

    print!("here");
    let new_user = NewUser::new(u_login, u_email, u_password);
    print!("Users: {:?}",new_user);
    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<Users>(conn)
        .expect("Error creating user in create_user");


    let link = crate::auth::generate_auth_link(conn);
    diesel::insert_into(activation_links)
    .values(NewActLink{
        user_id: user.id,
        reference: link.clone(),
    })
    .get_result::<ActLink>(conn)
    .expect("Error creating auth link");

    Ok(link)
}

use crate::auth::Login;

pub fn auth_user(login: Login, pass:String, conn: &PgConnection) -> Result<Users,()> {
    
    use crate::auth::make_password_hash;

    use Login::*;
    let users_vec = match login.clone() {
        Email(e) => {
            print!("trying to get user by email\n");
            users
            .filter(email.eq(e))
            .limit(1)
            .load::<Users>(conn)
            .expect("Error getting user auth by email")
        },
        Username(u) => {
            print!("uname: {}", u);
            print!("trying to get user by login!!!\n");
            users
            .filter(username.eq(u))
            .limit(1)
            .load::<Users>(conn)
            .expect("Error getting user auth by login")
        }
    };
    print!("vecs: {:?}", &users_vec);
    match (login,users_vec.get(0)) {
        (Email(e),Some(u_instance)) => {
            if u_instance.pass_hash == make_password_hash(e,u_instance.username.clone(),pass) {
                if u_instance.verifyed {
                    Ok(u_instance.clone())
                } else {
                    print!("not verifyed: can't log in");
                    Err(()) 
                }
            } else {
                print!("hashes do not match");
                Err(())
            }
        }
        (Username(u),Some(u_instance)) => {
            if u_instance.pass_hash == make_password_hash(u_instance.email.clone(),u.clone(),pass.clone()) {
                if u_instance.verifyed {
                    Ok(u_instance.clone())
                } else {
                    print!("not verifyed: can't log in");
                    Err(()) 
                }
            } else {
                print!("inst: {}\nclient: {}\n",u_instance.pass_hash,make_password_hash(u_instance.email.clone(),u,pass));
                print!("hashes do not match\n");
                Err(())
            } 
        }
        (_,None) => {
            print!("no such user in users auth user");
            Err(())
        }
    }
}


// допилить удаление всех таблиц связанных с юзером
pub fn delete_user(u_id: i32, conn: &PgConnection) {
    diesel::delete(users.filter(id.eq(u_id)))
    .execute(conn)
    .expect("Got an error while deleting user");
}

pub fn get_user_by_email(u_email: String, conn: &PgConnection) -> Users {
    let usr = users
        .filter(email.eq(u_email))
        .limit(5)
        .load::<Users>(conn)
        .expect("Error getting user by email in get_user_by_email");
    
    if let Some(u_instance) = usr.get(0) {
        u_instance.clone()
    } else {
        panic!("no user foun with email in cookies in get_user_by_email");
    }
}


