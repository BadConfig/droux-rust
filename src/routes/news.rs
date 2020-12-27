use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use serde::{
    Serialize,
    Deserialize,
};
use std::fs::File;
use std::io::prelude::*;
use crate::db::filters::get_filter_context;
use crate::Error;

extern crate rocket_multipart_form_data;
use rocket_multipart_form_data::{
    MultipartFormData, 
    MultipartFormDataField, 
    MultipartFormDataOptions,
};
use crate::users::CommonUser;
use crate::routes::Either;
use crate::routes::get_base_context;
use crate::models::product::ProductCard;
use rocket::http::Status;
use crate::models::product::NewProduct;
use crate::models::news::News;

#[get("/news/<id>")]
pub fn article(id: i32, user: CommonUser, conn: crate::db::Conn) -> Result<Template,Error> {
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("article", &News::get(id, &conn)?);
    Ok(Template::render("news/main", ctx))
}

#[get("/news/feed/products")]
pub fn products(user: CommonUser, conn: crate::db::Conn) -> Result<Template,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    let user_id = match user {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    ctx.insert("products", &ProductCard::in_news(user_id, &conn)?);
    Ok(Template::render("news/products", ctx))
}

#[get("/news/feed")]
pub fn feed(user: CommonUser, conn: crate::db::Conn) -> Result<Template,Error> {
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("articles", &News::pages(&conn)?);
    ctx.insert("banners", &News::banners(6, &conn)?);
    Ok(Template::render("news/feed", ctx))
}