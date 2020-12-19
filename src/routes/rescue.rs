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
use crate::models::rescue::Issue;

#[get("/rescue")]
pub fn rescue(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    Ok(Either::Template(Template::render("rescue", &ctx))) 
}

#[derive(FromForm,Debug,Clone)]
pub struct RescueForm {
    address: String,
    issue: String,
}

#[post("/rescue/create",data="<form>")]
pub fn create(form: Form<RescueForm>,  user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
      Issue::create(&form.address, &form.issue, &conn)?;
      Ok(Either::Redirect(Redirect::to("/")))
}