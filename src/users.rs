use crate::models::users::NewUser;

impl NewUser {
    pub fn new(u_login: String, u_email: String, u_pass: String) -> Self {
        use crate::auth::make_password_hash;
        NewUser {
            email: u_email.clone(),
            username: u_login.clone(),
            pass_hash: make_password_hash(u_email, u_login, u_pass),
        }
    }

}

use rocket;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use crate::models::users::Users;

#[derive(Clone)]
pub enum CommonUser {
    Logged(Users),
    NotLogged(),
}

use rocket::http::Cookie;

impl<'a, 'r> FromRequest<'a, 'r> for CommonUser {
 
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<CommonUser, ()> {
        
        use crate::auth::get_data_from_jwt;
        use crate::db::users::auth_user;
        let conn = &crate::db::establish_connection();

        let user_jwt = match request.cookies().get_private("user").map(| x: Cookie | x.value().to_string() ) {
            Some(s) => s,
            None => return Outcome::Success(CommonUser::NotLogged()),
        };
        let (logged,pass,_) = match get_data_from_jwt(user_jwt) {
            Some(d) => d,
            None => return Outcome::Success(CommonUser::NotLogged()),
        };
        print!("got jwt");
        let user_data = match auth_user(logged, pass, conn) {
            Ok(u) => u,
            _ => return Outcome::Success(CommonUser::NotLogged()),
        };

        drop(request.cookies());
        Outcome::Success(CommonUser::Logged(user_data))
    }
}
