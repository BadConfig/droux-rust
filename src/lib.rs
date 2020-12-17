#![feature(proc_macro_hygiene, decl_macro, type_ascription)]
pub mod schema;
pub mod models;
pub mod admin;
pub mod db;
pub mod auth;
pub mod routes;
pub mod users;
pub mod crm;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate rocket_slog;

use diesel::result::Error as DieselError;
use rocket::request::Request;
use rocket::http::Status;
use rocket::response;
use rocket::response::Responder;
use rocket_contrib::serve::StaticFiles;
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

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error {
            status: Status::InternalServerError,
            message: error.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error {
            status: Status::InternalServerError,
            message: error.to_string(),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Error {
        Error {
            status: Status::InternalServerError,
            message: error.to_string(),
        }
    }
}

use rocket_contrib::templates::{Template,tera::*};

pub fn app() -> rocket::Rocket {

    rocket::ignite()
        .mount("/",routes![
            routes::admin::admin_main,
            routes::admin::admin_product,
            routes::admin::product_change,
            routes::admin::admin_product_edit,
            routes::admin::admin_product_unpub,
            routes::auth::authorize,
            routes::auth::logout,
            routes::index,
            routes::auth::verify_link,
            routes::auth::register,
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
            routes::admin::admin_priveleges,
            routes::admin::admin_priveleges_add, 
            routes::admin::admin_priveleges_delete,
            routes::users::post_user_reviews_add,
            routes::product::get_promotions,
            routes::product::get_promotions_final,
            routes::product::post_promotions,
            routes::product::check_pay,
            routes::product::get_order_final,
            routes::product::get_order,
            routes::product::post_order,
            routes::static_pages::commission,
            routes::static_pages::contacts,
            routes::static_pages::criteria,
            routes::static_pages::faq,
            routes::static_pages::for_customer,
            routes::static_pages::for_seller,
            routes::static_pages::help,
            routes::static_pages::privacy_terms,
            routes::static_pages::save_deal,
            routes::static_pages::save_deal_terms,
            routes::static_pages::serve_terms,
            routes::static_pages::user_terms,
            routes::subs::unsubscribe,
            routes::subs::subscribe,
            routes::subs::get_sublist,
            routes::subs::unsubscribe_force,
            ])
        .attach(Template::fairing())
        .attach(db::Conn::fairing())

}

