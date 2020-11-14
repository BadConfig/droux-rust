use crate::db::users;
use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};

use rocket::State;
use crate::auth::IsLogged;
use std::sync::atomic::{Ordering,AtomicUsize};
use super::get_base_context;
use crate::users::CommonUser;
use crate::db::chat::*;
use crate::models::product::ProductCard;

use crate::routes::Either;
use crate::models::users::Users;

//#[get("/users/<id>")]
//pub fn get_product_by_id(user_id: i32, user: CommonUser, conn: crate::db::Conn) -> Template {

//} 

#[get("/users/favourites")]
pub fn get_users_favourites(user: CommonUser, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        ctx.insert("rating_floored", &(user.rate_summ/user.rate_count));
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        ctx.insert("favourite_products", &ProductCard::get_favourites(user.id.clone(), &conn));
        Either::Template(Template::render("users/favourites_content", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}

#[get("/users/products")]
pub fn get_users_products(user: CommonUser, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        ctx.insert("rating_floored", &(user.rate_summ/user.rate_count));
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Either::Template(Template::render("users/products_content", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
} 

#[get("/users/reviews")]
pub fn get_users_reviews(user: CommonUser, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        ctx.insert("rating_floored", &(user.rate_summ/user.rate_count));
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Either::Template(Template::render("users/reviews_content", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}