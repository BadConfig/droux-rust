#![feature(proc_macro_hygiene, decl_macro, type_ascription)]
pub mod schema;
pub mod models;
pub mod admin;
pub mod db;
pub mod auth;
pub mod routes;
pub mod users;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate rocket_slog;
use rocket_slog::SlogFairing;

use diesel::result::Error as DieselError;
use rocket::request::Request;
use rocket::http::Status;
use rocket::response;
use rocket::response::{Response, Responder};
//use std::error::Error;

#[derive(Debug)]
pub struct Error {
    status: Status,
    message: String,
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        print!("ERROR| {}\n",self.message);
        Err(self.status)
    }

}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        Error {
            status: Status::InternalServerError,
            message: error.to_string(),
        }
    }
}

use rocket_contrib::templates::{Template,tera::*};
use serde::Serialize;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::serve::{StaticFiles,Options};
use rocket::config::{Config, Environment, LoggingLevel};

pub fn app() -> rocket::Rocket {

    rocket::ignite()
        .mount("/",routes![
            routes::auth::login,
            routes::admin::admin_main,
            routes::admin::admin_product,
            routes::admin::product_change,
            routes::admin::admin_product_edit,
            routes::admin::admin_product_unpub,
            routes::auth::authorize,
            routes::auth::logout,
            routes::index,
            routes::auth::register,
            routes::auth::register_get,
            routes::auth::verify_link,
            routes::product::product_create,
            routes::product::product_create_get,
            routes::product::get_product_by_id,
            routes::product::file,
            routes::chat::write_chat_messages,
            routes::chat::create_chat_messages,
            routes::chat::get_chat_messages,
            routes::chat::get_chats,
            routes::filters::get_filter_data,
            routes::filters::get_filter_page,
            routes::users::get_users_products,
            routes::users::get_users_reviews,
            routes::users::get_users_favourites,
            routes::users::get_user_products_profile,
            routes::users::get_user_reviews_profile,
            routes::product::favourites_add,
            routes::product::favourites_delete,
            routes::admin::admin_users,
            routes::admin::admin_users_ban,
            routes::admin::admin_links,
            routes::admin::admin_links_change,
            ])
        .attach(Template::fairing())
        .attach(db::Conn::fairing())

}

