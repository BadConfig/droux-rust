pub mod admin;
pub mod auth;
pub mod product;
pub mod filters;
pub mod users;
pub mod chat;

use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use diesel::PgConnection;

use rocket::State;
use crate::auth::IsLogged;
use std::sync::atomic::{Ordering,AtomicUsize};
use crate::models::product::ProductCard;

use crate::users::CommonUser;

#[get("/")]
pub fn index(user: CommonUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user.clone(), &conn);
    let opt_id = match user.clone() {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    ctx.insert("most_viewed_products",&ProductCard::get_most_viewed(opt_id.clone(), &conn));
    ctx.insert("new_products", &ProductCard::get_recently_added(opt_id.clone(), &conn));
    ctx.insert("popular_seller_products", &ProductCard::get_by_seller_popular_products(opt_id.clone(), &conn));
    Template::render("index", &ctx )
}

use crate::models::users::Users;
pub enum UserGuard {
    Logged(Users),
    NotLogged,
}
pub fn get_required_context(data: UserGuard, _conn: &PgConnection) -> Context {
    let mut ctx = Context::new();
    use UserGuard::*;
    match data {
        Logged(usr) => {
            ctx.insert("is_logged", &true);
            ctx.insert("user", &usr);
        },
        NotLogged => ctx.insert("is_logged", &false),
    }

    ctx
}


pub fn get_base_context(usr: CommonUser, conn: &PgConnection) -> Context {

    use UserGuard::*;

    let usr_data = if let CommonUser::Logged(user_data) = usr {
        Logged(user_data)
    } else {
        NotLogged
    };

    get_required_context(usr_data, conn)

}

#[derive(Debug, Responder)]
pub enum Either {
    Template(Template),
    Redirect(Redirect),
}
