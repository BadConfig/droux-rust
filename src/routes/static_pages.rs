use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::{Template};
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
use rocket::Data;
use rocket::http::ContentType;
use std::sync::atomic::Ordering;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, 
    MultipartFormDataOptions, RawField, TextField, Repetition
};
use crate::users::CommonUser;
use crate::routes::Either;
use crate::routes::get_base_context;
use crate::models::product::ProductCard;
use crate::db::product::reviewed_by_user;
use rocket::http::Status;
use crate::models::product::NewProduct;

#[get("/for_customer")]
pub fn for_customer(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/for_customer", &ctx)))
}

#[get("/for_seller")]
pub fn for_seller(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/for_customer", &ctx)))
}

#[get("/user_terms")]
pub fn user_terms(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/user_terms", &ctx)))
}

#[get("/privacy_terms")]
pub fn privacy_terms(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/privacy_terms", &ctx)))
}

#[get("/serve_terms")]
pub fn serve_terms(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/serve_terms", &ctx)))
}

#[get("/save_deal")]
pub fn save_deal(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/save_deal", &ctx)))
}

#[get("/save_deal_terms")]
pub fn save_deal_terms(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/save_deal_terms", &ctx)))
}

#[get("/contacts")]
pub fn contacts(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/contacts", &ctx)))
}

#[get("/help")]
pub fn help(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/help", &ctx)))
}

#[get("/faq")]
pub fn faq(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/faq", &ctx)))
}

#[get("/criteria")]
pub fn criteria(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/criteria", &ctx)))
}

#[get("/commission")]
pub fn commission(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("static_pages/commission", &ctx)))
}

