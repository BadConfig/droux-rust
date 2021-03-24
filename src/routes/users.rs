use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use std::fs::File;
use std::io::prelude::*;

use super::get_base_context;
use crate::users::CommonUser;
use crate::models::product::ProductCard;
use crate::models::subs::Subscribes;

use crate::routes::Either;
use crate::models::users::Users;
use crate::models::product::Product;
use crate::Error;

extern crate rocket_multipart_form_data;
use rocket_multipart_form_data::{
    MultipartFormData, 
    MultipartFormDataField, 
    MultipartFormDataOptions,
};
use rocket::Data;
use rocket::http::ContentType;

#[get("/users/favourites")]
pub fn get_users_favourites(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        let rate_int = if user.rate_count != 0 {
            user.rate_summ/user.rate_count
        } else {
            0
        };
        let (you,yours) = Subscribes::count(user.id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("rating_floored", &rate_int);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        ctx.insert("favourite_products", &ProductCard::get_favourites(user.id.clone(), &conn));
        Ok(Either::Template(Template::render("users/favourites_content", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
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
        let (you,yours) = Subscribes::count(user.id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("active_products",&Product::get_products_by_status_and_user("published".into(),user.id, &conn)?);
        ctx.insert("sold_products",&Product::get_products_by_status_and_user("sold".into(),user.id, &conn)?);
        ctx.insert("deleted_products",&Product::get_products_by_status_and_user("deleted".into(),user.id, &conn)?);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Ok(Either::Template(Template::render("users/products_content", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
} 

#[get("/users/menu")]
pub fn get_user_menu_profile(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        let rate_int = if user.rate_count != 0 {
            user.rate_summ/user.rate_count
        } else {
            0
        };
        ctx.insert("rating_floored", &rate_int);
        let (you,yours) = Subscribes::count(user.id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("active_products",&Product::get_products_by_status_and_user("published".into(),user.id, &conn)?);
        ctx.insert("sold_products",&Product::get_products_by_status_and_user("sold".into(),user.id, &conn)?);
        ctx.insert("deleted_products",&Product::get_products_by_status_and_user("deleted".into(),user.id, &conn)?);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Ok(Either::Template(Template::render("users/menu_products_content", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
} 
#[get("/users/reviews")]
pub fn get_users_reviews(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    
    use crate::models::product::ProductRating;
    
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("my_user", &user.clone());
        let rate_int = if user.rate_count != 0 {
            user.rate_summ/user.rate_count
        } else {
            0
        };
        let (you,yours) = Subscribes::count(user.id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("reviews",&ProductRating::get_by_user(user.id,&conn));
        ctx.insert("rating_floored", &rate_int);
        ctx.insert("unread_messages", &crate::db::chat::having_unread(user.id.clone(), &conn));
        Ok(Either::Template(Template::render("users/reviews_content", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
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
        //TODO: add verifying that product is bought
        if user.id == form.u_id {
            return Ok(Either::Redirect(Redirect::to("/")));
        }
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
    if let CommonUser::Logged(u) = user {
        let (you,yours) = Subscribes::count(id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("in_subs", &Subscribes::exists(u.id, id.clone(), &conn)?);
        ctx.insert("prods", &Product::get_for_profile(id,u.id.clone(), &conn)?);
    } else {
        ctx.insert("in_subs", &false);
        ctx.insert("your_sub_count", &0);
        ctx.insert("you_sub_count", &0);
        let a: Vec<u8> = Vec::new();
        ctx.insert("prods", &a);
    }
    ctx.insert("viewed_user", &user_viewed.clone());
    ctx.insert("reviews",&ProductRating::get_by_user(id,&conn));
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
    if let CommonUser::Logged(u) = user {
        let (you,yours) = Subscribes::count(id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("in_subs", &Subscribes::exists(u.id, id.clone(), &conn)?);
        ctx.insert("prods", &Product::get_for_profile(id,u.id.clone(), &conn)?);
    } else {
        ctx.insert("in_subs", &false);
        ctx.insert("your_sub_count", &0);
        ctx.insert("you_sub_count", &0);
        let a: Vec<u8> = Vec::new();
        ctx.insert("prods", &a);
    }

    ctx.insert("active_products",&Product::get_products_by_status_and_user("published".into(),id, &conn)?);
    ctx.insert("sold_products",&Product::get_products_by_status_and_user("sold".into(),id, &conn)?);
    ctx.insert("rating_floored", &rate_int);
    Ok(Either::Template(Template::render("profile/products", &ctx)))

}

#[get("/profile/users/<id>/profile")]
pub fn get_user_main_profile(id: i32, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    
    let mut ctx = get_base_context(user.clone(), &conn);
    let user_viewed = Users::get_by_id(id, &conn);
    ctx.insert("viewed_user", &user_viewed.clone());
    let rate_int = if user_viewed.rate_count != 0 {
        user_viewed.rate_summ/user_viewed.rate_count
    } else {
        0
    };
    if let CommonUser::Logged(u) = user {
        let (you,yours) = Subscribes::count(id.clone(), &conn)?;
        ctx.insert("your_sub_count", &yours);
        ctx.insert("you_sub_count", &you);
        ctx.insert("in_subs", &Subscribes::exists(u.id, id.clone(), &conn)?);
        ctx.insert("prods", &Product::get_for_profile(id,u.id.clone(), &conn)?);
    } else {
        ctx.insert("in_subs", &false);
        ctx.insert("your_sub_count", &0);
        ctx.insert("you_sub_count", &0);
        let a: Vec<u8> = Vec::new();
        ctx.insert("prods", &a);
    }

    ctx.insert("active_products",&Product::get_products_by_status_and_user("published".into(),id, &conn)?);
    ctx.insert("sold_products",&Product::get_products_by_status_and_user("sold".into(),id, &conn)?);
    ctx.insert("rating_floored", &rate_int);
    Ok(Either::Template(Template::render("profile/products_main", &ctx)))

}

#[post("/users/profile_pictures/delete")]
pub fn rm_user_image(
    user: CommonUser, 
    conn: crate::db::Conn
) -> Result<Either,Error> {
    if let CommonUser::Logged(u) = user {
        u.set_photo("default_picture.png".to_string(), &conn)?;
        return Ok(Either::Redirect(Redirect::to("/users/menu")))
    } 
    Ok(Either::Redirect(Redirect::to("/")))
}

#[post("/users/profile_pictures/create",data="<form>")]
pub fn add_user_image(
    content_type: &ContentType, 
    form: Data, 
    user: CommonUser, 
    conn: crate::db::Conn
) -> Result<Either,Error> {
    if let CommonUser::Logged(u) = user {
        let fname = parse_image_multiform(content_type, form, u.username.clone());
        u.set_photo(fname, &conn)?;
        return Ok(Either::Redirect(Redirect::to("/users/menu")))
    } 
    Ok(Either::Redirect(Redirect::to("/")))
}

pub fn parse_image_multiform(content_type: &ContentType, form: Data, uname: String) -> String {
    
    use crate::routes;

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            //.content_type_by_string(Some(mime::IMAGE_STAR)).unwrap()
            MultipartFormDataField::raw("profile_photo")
                .size_limit(16*1920*1080),
        ]
    );
    let mut multipart_form_data = MultipartFormData::parse(content_type, form, options).unwrap();
    let profile_photo = multipart_form_data.raw.get("profile_photo"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    fn unpack_photo(
        photo: &Option<&std::vec::Vec<routes::users::rocket_multipart_form_data::RawField>>, 
        title: &String
    ) -> Result<String,()> {
        let ph = match photo {
            Some(d) => &d[0],
            None => return Err(()),
        };
        let ph = ph.raw.clone();
        let store_path = format!("static/profile_pictures/{}.jpg",title);
        let mut file = File::create(&store_path).expect("can't create file");
        file.write_all(&ph).expect("error wfiting photo to file");
        Ok(title.clone())
    }

    unpack_photo(&profile_photo, &(chrono::Local::now().naive_utc().to_string()+uname.as_ref()))
        .expect("error while downloading image")
}
