use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use std::fs::File;
use std::io::prelude::*;

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


#[get("/product/<id>")]
pub fn get_product_by_id(id: i32, user: CommonUser, conn: crate::db::Conn) -> Template {
    
    use crate::db::users::get_user_by_email;
    use crate::models::product::ProductContext;
    use crate::db::product::{get_brand_list,get_brand_name,get_product_data};

    let mut ctx = get_base_context(user, &conn);
    ctx.insert("brands", &get_brand_list(&conn));

    let product = get_product_data(id, &conn);
    let product = ProductContext {
        category_name: get_brand_name(product.category_id,&conn).name,
        id: product.id,
        title: product.title,
        descr: product.descr,
        price: product.price,
        location: product.location,
        state: product.state,
        category_id: product.category_id,
        seller_id: product.seller_id,
        pictures: product.pictures,
        create_datetime: product.create_datetime,
    };
    ctx.insert("product", &product);
    ctx.insert("brands", &get_brand_list(&conn));
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
   
    use crate::db::product::get_brand_list;

    let mut ctx = get_base_context(user.clone(), &conn);
    ctx.insert("brands", &get_brand_list(&conn));

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
                .size_limit(16*1024*1024)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap(),
            MultipartFormDataField::raw("photo2")
                .size_limit(16*1024*1024),
            MultipartFormDataField::raw("photo3")
                .size_limit(16*1024*1024),
            MultipartFormDataField::raw("photo4")
                .size_limit(16*1024*1024),
            MultipartFormDataField::raw("photo2")
                .size_limit(16*1024*1024),
            MultipartFormDataField::text("title"),
            MultipartFormDataField::text("descr"),
            MultipartFormDataField::text("size"),
            MultipartFormDataField::text("state"),
            MultipartFormDataField::text("price"),   
            MultipartFormDataField::text("location"),         
            MultipartFormDataField::text("seller_id"),
            MultipartFormDataField::text("category_id"),
        ]
    );


    let mut multipart_form_data = MultipartFormData::parse(content_type, form, options).unwrap();

    let photo1 = multipart_form_data.raw.get("photo1"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let photo2 = multipart_form_data.raw.get("photo2");
    let photo3 = multipart_form_data.raw.get("photo3");
    let photo4 = multipart_form_data.raw.get("photo4");
    let photo5 = multipart_form_data.raw.get("photo5");
    let title = multipart_form_data.texts.remove("title"); // Use the remove method to move text fields out of the MultipartFormData instance (recommended)
    let email = multipart_form_data.texts.remove("email");
    let descr = multipart_form_data.texts.remove("descr");
    let size = multipart_form_data.texts.remove("size");
    let price = multipart_form_data.texts.remove("price");
    let location = multipart_form_data.texts.remove("location");
    let state = multipart_form_data.texts.remove("state");
    let seller_id = multipart_form_data.texts.remove("seller_id");
    let category_id = multipart_form_data.texts.remove("category_id");

    fn unpack(data: Option<std::vec::Vec<routes::product::rocket_multipart_form_data::TextField>>) -> String {
                match data {
                    Some(mut d) => d.remove(0).text,
                    None => panic!("error: field does not exist"),
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
    let size    = unpack(size);
    let state   = unpack(state);
    let location= unpack(location);

    let price       : i32 = unpack(price).trim().parse().expect("error parsing price");
    let seller_id   : i32 = unpack(seller_id).trim().parse().expect("error parsing seller_id");
    let category_id : i32 = unpack(category_id).trim().parse().expect("error parsing category_id");

    let mut photos_array = Vec::new();
    if !unpack_photo(&photo1, &mut photos_array, &title,1) {
        panic!("can't load product without photo");
    }
    unpack_photo(&photo2, &mut photos_array, &title,2);
    unpack_photo(&photo3, &mut photos_array, &title,3);
    unpack_photo(&photo4, &mut photos_array, &title,4);
    unpack_photo(&photo5, &mut photos_array, &title,5);
    
    use crate::db::product::create_product;
    use crate::models::product::NewProduct;
    let new_product = NewProduct {
        title: title,
        descr: descr,
        price: price,
        location: location,
        state: state,
        category_id: category_id,
        seller_id: seller_id,
        pictures: photos_array,
    };
    create_product(new_product, &conn);
    Redirect::to("/")
}