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
use rocket::Data;
use rocket::http::ContentType;
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

#[post("/news_add",data="<form>")]
pub fn news_add(form: Data, content_type: ContentType, user: CommonUser, conn: crate::db::Conn) -> Result<Status,Error> {

}