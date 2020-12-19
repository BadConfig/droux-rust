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


#[get("/product/<id>")]
pub fn get_product_by_id(id: i32, user: CommonUser, conn: crate::db::Conn) -> Template {
    
    use crate::models::product::ProductContext;
    use crate::db::product::get_brand_list;

    crate::db::product::increment_product_views(id, &conn);
    crate::db::product::increment_product_today_views(id, &conn);
    let opt_id = match user.clone() {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    let mut ctx = get_base_context(user.clone(), &conn);
    ctx.insert("brands", &get_brand_list(&conn));
    ctx.insert("most_viewed_products", &ProductCard::get_recently_added(8, opt_id.clone(), &conn));

    let user_id = match user {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    let (product,seller) = ProductContext::get_by_id(id, user_id, &conn);
    ctx.insert("product", &product);
    if seller.rate_count == 0 {
        ctx.insert("rating_floored", &0);
    } else {
        ctx.insert("rating_floored", &(seller.rate_summ / seller.rate_count)); 
    }
    ctx.insert("seller", &seller);
    Template::render("product/main", &ctx)
}

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
// use rocket::State;
// extern crate rocket_file_cache;
// use rocket_file_cache::{Cache, CacheBuilder, CachedFile};

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/product/create")]
pub fn product_create_get(user: CommonUser, conn: crate::db::Conn) -> Either {

    let mut ctx = get_base_context(user.clone(), &conn);
    get_filter_context(&mut ctx, &conn);

    if let CommonUser::Logged(_) = user {
            Either::Template(Template::render("product/create", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}

pub fn parse_multiform_product(content_type: &ContentType, form: Data) -> NewProduct {
    
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
    print!("SELLER ID| {}",&seller_id);
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
    
    NewProduct {
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
    }
}

#[post("/product/create",data="<form>")]
pub fn product_create(content_type: &ContentType, form: Data, user: CommonUser, conn: crate::db::Conn) ->  String {
   
    use crate::db::product::create_product;
    if let CommonUser::Logged(_) = user {
        let p = parse_multiform_product(content_type, form);
        let id = create_product(p, &conn);
        return id;
    }
    "-1".to_string()
}

#[derive(FromForm,Clone)]
pub struct FavouritesForm {
    prod_id: i32,
}

#[post("/product/favourites/add",data="<form>")]
pub fn favourites_add(form: Form<FavouritesForm>, user: CommonUser, conn: crate::db::Conn) -> Result<Status,Error> {
    
    use crate::models::product::FavouriteProducts;

    if let CommonUser::Logged(user) = user {
        FavouriteProducts::add_favourite(user.id, form.prod_id, &conn)?;
        return Ok(Status::Ok);
    }
    Ok(Status::Unauthorized)
}

#[post("/product/favourites/delete",data="<form>")]
pub fn favourites_delete(form: Form<FavouritesForm>, user: CommonUser, conn: crate::db::Conn) -> Result<Status,Error> {
    
    use crate::models::product::FavouriteProducts;
    
    if let CommonUser::Logged(user) = user {
        FavouriteProducts::delete_favourite(user.id, form.prod_id, &conn)?;
        return Ok(Status::Ok);
    }
    Ok(Status::Unauthorized)
}

use crate::models::product::Product;
#[get("/product/promotion/create/<id>")]
pub fn get_promotions(id: i32, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    use crate::models::product::ProductPromotions;
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        let pr = Product::get_by_id(id, &conn);
        if pr.seller_id != user.id || ProductPromotions::exists(id.clone(), &conn)? {
            return Ok(Either::Redirect(Redirect::to("/")));
        }
        ctx.insert("product", &pr);
        ctx.insert("seller_id", &user.id);
        Ok(Either::Template(Template::render("product/promotion", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
}

#[derive(FromForm,Clone,Serialize,Deserialize,Debug)]
pub struct BuyForm {
    pub name: String,
    pub username: String,
    pub mail: String,
    pub phone: String,
    pub post_index: String,
    pub ship_addr: String,
    pub pr_price: i32,
    pub seller_username: String,
    pub seller_phone: String,
    pub seller_id: i32,
    pub seller_email: String,
    pub seller_location: String,
    pub pr_is_pre_order: bool,
    pub pr_id: i32,
    pub pr_name: String,
}

#[derive(FromForm,Clone,Serialize,Deserialize,Debug)]
pub struct PrivForm {
    pub pre_order: bool,
    pub top_name: bool,
    pub top_cat: bool,
    pub take_in_news: bool,
    pub all: bool,
    pub none: bool,
    pub seller_id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub promo: String,
}

use crate::crm::{
    TrDescription,
};

#[get("/product/pay?<orderId>&<lang>")]
pub fn check_pay(orderId: String, lang: String, conn: crate::db::Conn) -> Result<Either,Error> {
    let transcation = TrDescription::get_sber_pay_status(orderId)?;
    print!("\nINFO| {:?}\n",&transcation);
    match transcation {
        TrDescription::Priveleges(p) => {
            p.save(&conn)?;
            print!("\nINFO| got in priveleges\n");
            Ok(Either::Redirect(Redirect::to("/product/promotion/final")))
        },
        TrDescription::Order(o) => {
            let (num, addr) = o.send_new_order_to_crm()?;
            Product::set_status(o.pr_id.clone(), "sold".to_string(), &conn)?;
            Product::set_customer_id(o.pr_id,o.seller_id.clone(), &conn)?;
            print!("\nINFO| got in order\n");
            Ok(Either::Redirect(Redirect::to(format!("/product/order/final/{}/{}",num,addr))))
        },
        _ => Ok(Either::Redirect(Redirect::to("/")))
    }
}
use crate::models::users::Users;
use crate::models::product::ProductPromotions;
#[get("/product/order/create/<prod_id>")]
pub fn get_order(prod_id: i32, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        let pr = Product::get_by_id(prod_id, &conn);
        ctx.insert("seller", &Users::get_by_id(pr.seller_id, &conn));
        ctx.insert("is_pre_order", &ProductPromotions::get_pre_order(pr.id, &conn)?);
        ctx.insert("product", &pr);
        ctx.insert("my_user", &user);
        Ok(Either::Template(Template::render("product/order", &ctx)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
}

#[post("/product/order/create",data="<form>")]
pub fn post_order(form: Form<BuyForm>, user: CommonUser) -> Result<Either,Error> {
    if let CommonUser::Logged(_) = user {
        let url = form.send_sber_pre_pay_link()?;
        return Ok(Either::Redirect(Redirect::to(url)));
    }
    Ok(Either::Redirect(Redirect::to("/")))
}

#[post("/product/promotion/create",data="<form>")]
pub fn post_promotions(form: Form<PrivForm>, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    use crate::models::promos::Promo;
    if let CommonUser::Logged(user) = user {
        if form.seller_id != user.id {
            return Ok(Either::Redirect(Redirect::to("/")));
        }
        if form.none {
            return Ok(Either::Redirect(Redirect::to("/product/promotion/final")));
        }
        let mut summ: i64 = if form.all || (form.take_in_news && form.top_cat && form.top_name) {
            599
        } else if form.take_in_news && form.top_cat {
            399 + 149
        } else if form.take_in_news && form.top_name {
            399 + 189
        } else if form.top_name && form.top_cat {
            189 + 149
        } else if form.take_in_news {
            399
        } else if form.top_cat {
            149
        } else if form.top_name {
            189
        } else {
            0
        };
        if form.pre_order {
            summ += 499;
        }
        summ = Promo::get_sale(summ, form.promo.clone(), &conn);
        let url = form
            .into_inner()
            .send_sber_pay_link(summ)?;
        print!("\n{}\n",url);
        Ok(Either::Redirect(Redirect::to(url)))
    } else {
        Ok(Either::Redirect(Redirect::to("/")))
    }
}

#[get("/product/promotion/final")]
pub fn get_promotions_final(user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let ctx = get_base_context(user.clone(), &conn);
    Ok(Either::Template(Template::render("product/promotion_final", &ctx)))
}

#[get("/product/order/final/<num>/<addr>")]
pub fn get_order_final(num: String, addr: String, user: CommonUser, conn: crate::db::Conn) -> Result<Either,Error> {
    let mut ctx = get_base_context(user.clone(), &conn);
    ctx.insert("order_number", &num);
    ctx.insert("order_adress", &addr);
    Ok(Either::Template(Template::render("product/order_final", &ctx)))
}