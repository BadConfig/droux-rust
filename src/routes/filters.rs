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


#[get("/filters?<search_string>&<prod_size_id>&<product_state_id>&\
<limit>&<offset>&<subcategory_id>&<category_id>&<prod_brand_id>&\
<prod_type_id>&<order_by>&<user_id>")]
pub fn get_filter_page(search_string: Option<String>,
        prod_size_id: Option<i32>,
        product_state_id: Option<i32>,
        limit: i32,
        offset: i32,
        subcategory_id: Option<i32>,
        category_id: Option<i32>,
        prod_brand_id: Option<i32>,
        prod_type_id: Option<i32>,
        order_by: Option<String>,
        user_id: Option<i32>, 
        user: CommonUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user, &conn);
    crate::db::filters::get_filter_context(&mut ctx, &conn);
    let form = SearchForm {
        search_string: search_string,
        prod_size_id: prod_size_id,
        product_state_id: product_state_id,
        limit: limit,
        offset: offset,
        subcategory_id: subcategory_id,
        category_id: category_id,
        prod_brand_id: prod_brand_id,
        prod_type_id: prod_type_id,
        order_by: order_by,
        user_id: user_id, 
    };
    ctx.insert("FoundProducts",&ProductCard::filter_search(form, &conn));
    Template::render("filters", &ctx)
}