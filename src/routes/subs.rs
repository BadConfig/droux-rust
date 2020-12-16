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
use crate::models::subs::Subscribes;

#[get("/lk/subscribes")]
pub fn get_sublist(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(u) = user {
        let (you,yours) = Subscribes::count(u.id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("your_sub_users", &Subscribes::subscribers(u.id.clone(), 30, 0, &conn)?);
        ctx.insert("you_sub_users", &Subscribes::subscribed(u.id.clone(), 30, 0, &conn)?);

        Ok(Either::Template(Template::render("users/subs", ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }  
}

#[derive(FromForm,Debug,Clone)]
pub struct SubscribeForm {
    u_id: i32,
}

#[post("/subs/new",data="<form>")]
pub fn subscribe(form: Form<SubscribeForm>,  user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if let CommonUser::Logged(u) = user {
        Subscribes::create(u.id, form.u_id.clone(), &conn)?;
        Ok(Either::Redirect(Redirect::to(format!("/profile/users/{}/reviews",form.u_id))))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }  
}

#[post("/subs/unsub",data="<form>")]
pub fn unsubscribe(form: Form<SubscribeForm>,  user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if let CommonUser::Logged(u) = user {
        Subscribes::delete(u.id, form.u_id.clone(), &conn)?;
        Ok(Either::Redirect(Redirect::to(format!("/profile/users/{}/reviews",form.u_id))))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }  
}
