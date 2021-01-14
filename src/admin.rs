use serde::Serialize;

pub enum UserPriveleges {
    Admin,
    Moderator,
    Editor,
}

impl UserPriveleges  {
    pub fn get_string(&self) -> String {
        match self {
            UserPriveleges::Admin => "admin".to_string(),
            UserPriveleges::Moderator => "moderator".to_string(),
            UserPriveleges::Editor => "editor".to_string(),
        }
    }

    pub fn set_by_string(string: String) -> Result<UserPriveleges,()> {
        match &string[..] {
            "admin" => Ok(UserPriveleges::Admin),
            "moderator" => Ok(UserPriveleges::Moderator),
            "editor" => Ok(UserPriveleges::Editor),
            _ => {
                print!("Error converting string ot user priveleges");
                Err(())
            }
        }
    }

}

use rocket;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

fn get_priveleges_from_cookies(request: &Request) -> Option<UserPriveleges> {
    use crate::auth::get_data_from_jwt;
    use crate::db::admin::get_priveleges;
    use crate::db::users::auth_user;
    let conn = &crate::db::establish_connection();

    let user_jwt = match request
            .cookies()
            .get_private("user")
            .map(| x | x.value().to_string() ) {
        Some(s) => s,
        None => return None,
    };
    
    let (logged,pass,_) = match get_data_from_jwt(user_jwt) {
        Some(d) => d,
        None => return None,
    };

    let user_data = match auth_user(logged, pass, conn) {
        Ok(u) => u,
        _ => return None,
    };

    Some(get_priveleges(user_data.id, conn))
}

#[derive(Serialize)]
pub struct AdminUser {
    pub is_admin: bool,
    pub is_moderator: bool,
    pub is_editor: bool,
}
impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
 
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, ()> {
        
        use UserPriveleges::*;
        match get_priveleges_from_cookies(request) {
            Some(Admin) => Outcome::Success(AdminUser{
                is_admin: true,
                is_moderator: false,
                is_editor: false,
            }),
            Some(Moderator) => Outcome::Success(AdminUser{
                is_admin: false,
                is_moderator: true,
                is_editor: false,
            }),
            Some(Editor) => Outcome::Success(AdminUser{
                is_admin: false,
                is_moderator: false,
                is_editor: true,
            }),
            None => {
                print!("no priveleges found");
                Outcome::Forward(())
            }
        }
    }
}

 



