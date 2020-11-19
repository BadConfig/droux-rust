use crate::db::users;
use rocket_contrib::templates::{Template,tera::Context};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};

use rocket::State;
use crate::auth::IsLogged;
use std::sync::atomic::{Ordering,AtomicUsize};
use super::get_base_context;
use crate::users::CommonUser;
use crate::db::chat::*;
use crate::models::product::ProductCard;

use crate::routes::Either;
use crate::models::users::Users;
use crate::models::product::Product;
use crate::Error;

//#[get("/users/<id>")]
//pub fn get_product_by_id(user_id: i32, user: CommonUser, conn: crate::db::Conn) -> Template {

//} 

#[get("/users/favourites")]
pub fn get_users_favourites(user: CommonUser, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        let rate_int = if user.rate_count != 0 {
            user.rate_summ/user.rate_count
        } else {
            0
        };
        ctx.insert("rating_floored", &rate_int);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        ctx.insert("favourite_products", &ProductCard::get_favourites(user.id.clone(), &conn));
        Either::Template(Template::render("users/favourites_content", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}

#[get("/users/products")]
pub fn get_users_products(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        let rate_int = if user.rate_count != 0 {
            user.rate_summ/user.rate_count
        } else {
            0
        };
        ctx.insert("rating_floored", &rate_int);
        
        ctx.insert("active_products",&Product::get_products_by_status_and_user("published".into(),user.id, &conn)?);
        ctx.insert("sold_products",&Product::get_products_by_status_and_user("sold".into(),user.id, &conn)?);
        ctx.insert("deleted_products",&Product::get_products_by_status_and_user("deleted".into(),user.id, &conn)?);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Ok(Either::Template(Template::render("users/products_content", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
} 

#[get("/users/reviews")]
pub fn get_users_reviews(user: CommonUser, conn: crate::db::Conn) -> Either {
    
    use crate::models::product::ProductRating;
    
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        let rate_int = if user.rate_count != 0 {
            user.rate_summ/user.rate_count
        } else {
            0
        };
        ctx.insert("reviews",&ProductRating::get_by_user(user.id,&conn));
        ctx.insert("rating_floored", &rate_int);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Either::Template(Template::render("users/reviews_content", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}
#[derive(FromForm,Clone)]
pub struct NewReview {
    pr_id: i32,
    status: String,
    rate: i16,
    descr: String,
    u_id: i32,
}


#[post("/profile/users/reviews/add", data="<form>")]
pub fn post_user_reviews_add(form: Form<NewReview>, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    
    use crate::models::product::ProductRating;

    if let CommonUser::Logged(user) = user {
        ProductRating::set_rating(
            user.id, 
            form.u_id, 
            form.pr_id, 
            form.rate, 
            form.descr.clone(), 
            form.status.clone(), 
            &conn);
        Ok(Either::Redirect(Redirect::to(format!("/profile/users/{}/reviews",form.u_id))))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
}

#[get("/profile/users/<id>/reviews")]
pub fn get_user_products_profile(id: i32, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    
    use crate::models::product::ProductRating;

    let mut ctx = get_base_context(user.clone(), &conn);
    let user_viewed = Users::get_by_id(id, &conn);
    ctx.insert("viewed_user", &user_viewed.clone());
    ctx.insert("reviews",&ProductRating::get_by_user(id,&conn));
    ctx.insert("prods", &Product::get_for_profile(id, &conn)?);
    let rate_int = if user_viewed.rate_count != 0 {
        user_viewed.rate_summ/user_viewed.rate_count
    } else {
        0
    };
    ctx.insert("reviews",&ProductRating::get_by_user(id,&conn));
    ctx.insert("rating_floored", &rate_int);
    Ok(Either::Template(Template::render("profile/reviews", &ctx)))

}

#[get("/profile/users/<id>/product")]
pub fn get_user_reviews_profile(id: i32, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    
    let mut ctx = get_base_context(user.clone(), &conn);
    let user_viewed = Users::get_by_id(id, &conn);
    ctx.insert("viewed_user", &user_viewed.clone());
    let rate_int = if user_viewed.rate_count != 0 {
        user_viewed.rate_summ/user_viewed.rate_count
    } else {
        0
    };
    ctx.insert("prods", &Product::get_for_profile(id, &conn)?);
    ctx.insert("active_products",&Product::get_products_by_status_and_user("published".into(),id, &conn)?);
    ctx.insert("sold_products",&Product::get_products_by_status_and_user("sold".into(),id, &conn)?);
    ctx.insert("rating_floored", &rate_int);
    Ok(Either::Template(Template::render("profile/products", &ctx)))

}