use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use std::fs::File;
use std::io::prelude::*;
use crate::db::filters::get_filter_context;

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


#[get("/product/<id>")]
pub fn get_product_by_id(id: i32, user: CommonUser, conn: crate::db::Conn) -> Template {
    
    use crate::db::users::get_user_by_email;
    use crate::models::product::ProductContext;
    use crate::db::product::{get_brand_list,get_brand_name,get_product_data};

    let opt_id = match user.clone() {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    let mut ctx = get_base_context(user.clone(), &conn);
    ctx.insert("brands", &get_brand_list(&conn));
    ctx.insert("most_viewed_products", &ProductCard::get_most_viewed(8, opt_id.clone(), &conn));

    let product = get_product_data(id, &conn);

    let user_id = match user {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    let (product,seller) = ProductContext::get_by_id(id, user_id, &conn);
    ctx.insert("product", &product);
    ctx.insert("seller", &seller);
    Template::render("product/main", &ctx)
}

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/product/create")]
pub fn product_create_get(user: CommonUser, conn: crate::db::Conn) -> Either {

    let mut ctx = get_base_context(user.clone(), &conn);
    get_filter_context(&mut ctx, &conn);

    if let CommonUser::Logged(user_data) = user {
            Either::Template(Template::render("product/create", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}

#[post("/product/create",data="<form>")]
pub fn product_create(content_type: &ContentType, form: Data, user: CommonUser, conn: crate::db::Conn) -> Redirect {
   
    use crate::routes;
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            //.content_type_by_string(Some(mime::IMAGE_STAR)).unwrap()
            MultipartFormDataField::raw("photo1")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo2")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo3")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo4")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo5")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo6")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo7")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo8")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo9")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("photo10")
                .size_limit(16*1920*1080),
            MultipartFormDataField::text("title"),
            MultipartFormDataField::text("sub_category_id"),
            MultipartFormDataField::text("descr"),
            MultipartFormDataField::text("size_id"),
            MultipartFormDataField::text("state_id"),
            MultipartFormDataField::text("price"),   
            MultipartFormDataField::text("location"),         
            MultipartFormDataField::text("seller_id"),
            MultipartFormDataField::text("brand_id"),
            MultipartFormDataField::text("type_id"),
            MultipartFormDataField::text("phone_number"),
        ]
    );


    let mut multipart_form_data = MultipartFormData::parse(content_type, form, options).unwrap();

    let photo1 = multipart_form_data.raw.get("photo1"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let photo2 = multipart_form_data.raw.get("photo2");
    let photo3 = multipart_form_data.raw.get("photo3");
    let photo4 = multipart_form_data.raw.get("photo4");
    let photo5 = multipart_form_data.raw.get("photo5");
    let photo6 = multipart_form_data.raw.get("photo6"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let photo7 = multipart_form_data.raw.get("photo7");
    let photo8 = multipart_form_data.raw.get("photo8");
    let photo9 = multipart_form_data.raw.get("photo9");
    let photo10 = multipart_form_data.raw.get("photo10");
    let title = multipart_form_data.texts.remove("title"); // Use the remove method to move text fields out of the MultipartFormData instance (recommended)
    let descr = multipart_form_data.texts.remove("descr");
    let sub_category_id = multipart_form_data.texts.remove("sub_category_id");
    let size_id = multipart_form_data.texts.remove("size_id");
    let price = multipart_form_data.texts.remove("price");
    let location = multipart_form_data.texts.remove("location");
    let state_id = multipart_form_data.texts.remove("state_id");
    let seller_id = multipart_form_data.texts.remove("seller_id");
    let brand_id = multipart_form_data.texts.remove("brand_id");
    let type_id = multipart_form_data.texts.remove("type_id");
    let phone_number = multipart_form_data.texts.remove("phone_number");


    fn unpack(data: Option<std::vec::Vec<routes::product::rocket_multipart_form_data::TextField>>) -> String {
                match data {
                    Some(mut d) => d.remove(0).text,
                    None => panic!("error: field does not exist "),
                }
    }
    fn unpack_photo(photo: &Option<&std::vec::Vec<routes::product::rocket_multipart_form_data::RawField>>, paths: &mut Vec<String>, title: &String, num: i32) -> bool {
        let ph = match photo {
            Some(d) => &d[0],
            None => return false,
        };
        let ph = ph.raw.clone();
        let store_path = format!("static/product_pictures/{}-{}-{}.jpg",chrono::Local::now().naive_utc(),title,num);
        let mut file = File::create(&store_path).expect("can't create file");
        file.write_all(&ph).expect("error wfiting photo to file");
        paths.push(store_path);
        true
    }

    let title   = unpack(title);
    let descr   = unpack(descr);
    let location= unpack(location);
    let phone_number= unpack(phone_number);


    let price       : i32 = unpack(price).trim().parse().expect("error parsing price");
    let seller_id   : i32 = unpack(seller_id).trim().parse().expect("error parsing seller_id");
    let brand_id    : i32 = unpack(brand_id).trim().parse().expect("error parsing category_id");
    let sub_category_id : i32 = unpack(sub_category_id).trim().parse().expect("error parsing category_id");
    let size_id     : i32 = unpack(size_id).trim().parse().expect("error parsing category_id");
    let state_id    : i32 = unpack(state_id).trim().parse().expect("error parsing category_id");
    let type_id     : i32 = unpack(type_id).trim().parse().expect("error parsing category_id");


    let mut photos_array = Vec::new();
    if !unpack_photo(&photo1, &mut photos_array, &title,1) {
        panic!("can't load product without photo");
    }
    unpack_photo(&photo2, &mut photos_array, &title,2);
    unpack_photo(&photo3, &mut photos_array, &title,3);
    unpack_photo(&photo4, &mut photos_array, &title,4);
    unpack_photo(&photo5, &mut photos_array, &title,5);
    unpack_photo(&photo6, &mut photos_array, &title,6);
    unpack_photo(&photo7, &mut photos_array, &title,7);
    unpack_photo(&photo8, &mut photos_array, &title,8);
    unpack_photo(&photo9, &mut photos_array, &title,9);
    unpack_photo(&photo10, &mut photos_array, &title,10);
    
    
    use crate::db::product::create_product;
    use crate::models::product::NewProduct;
    let new_product = NewProduct {
        sub_category_id: sub_category_id,
        title: title,
        descr: descr,
        price: price,
        location: location,
        product_state: state_id,
        brand_id: brand_id,
        seller_id: seller_id,
        pictures: photos_array,
        type_id: type_id,
        size_id: size_id,
        phone_number: phone_number,
    };
    create_product(new_product, &conn);
    Redirect::to("/")
}


#[derive(FromForm,Clone)]
pub struct StarsForm {
    toid: i32,
    strs: i16,
}
#[post("/users/rating",data="<form>")]
pub fn rating_add(form: Form<StarsForm>, user: CommonUser, conn: crate::db::Conn) -> Redirect {
    use crate::models::product::ProductRating;
    if let CommonUser::Logged(user) = user {
        ProductRating::set_rating(user.id,form.toid,form.strs, &conn);
    }
    Redirect::to("/")
}