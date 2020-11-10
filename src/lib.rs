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

use rocket_contrib::templates::{Template,tera::*};
use serde::Serialize;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::serve::{StaticFiles,Options};

use std::sync::atomic::AtomicUsize;

pub fn app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",routes![
            routes::auth::login,
            routes::admin::admin_main,
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
            routes::users::get_users_me_main,
            routes::users::get_users_favourites,
            ])
        .attach(Template::fairing())
        .attach(db::Conn::fairing())

}

