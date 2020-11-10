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

use crate::auth::make_jwt_for_user;
use crate::routes::Either;
use crate::models::users::Users;
use crate::db::filters::{
    get_filter_context,
};
use rocket::response::content;
use rocket_contrib::json::Json;

use crate::models::product::{
    ProductCard,
    SearchForm,
};

#[post("/filters/lots",data="<form>")]
pub fn get_filter_data(form: Form<SearchForm>, user: CommonUser, conn: crate::db::Conn) -> Json<Vec<ProductCard>> {
    let f = ProductCard::filter_search(form.into_inner(), &conn);
    print!("\n{:?}\n",&f);
    Json(f)
}

#[post("/filters",data="<form>")]
pub fn get_filter_page(form: Form<SearchForm>, user: CommonUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user, &conn);
    crate::db::filters::get_filter_context(&mut ctx, &conn);
    ctx.insert("FoundProducts",&false);
    Template::render("filters", &ctx)
}