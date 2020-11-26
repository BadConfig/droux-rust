use chrono::NaiveDateTime;
extern crate serde;
use crate::Error;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    products,
    favourites,
    promotions,
    rating,
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable, SmallInt,
};
use crate::models::users::Users;

#[derive(Serialize, Deserialize,QueryableByName)]
pub struct ProductContext {
    #[sql_type="Integer"]
    pub id: i32,
    #[sql_type="Text"]
    pub title: String,
    #[sql_type="Text"]
    pub descr: String,
    #[sql_type="Integer"]
    pub sub_category_id: i32,
    #[sql_type="Text"]
    pub sub_category_name: String,
    #[sql_type="Integer"]
    pub category_id: i32,
    #[sql_type="Text"]
    pub category_name: String,
    #[sql_type="Integer"]
    pub state_id: i32,
    #[sql_type="Text"]
    pub state_name: String,
    #[sql_type="Integer"]
    pub seller_id: i32,
    #[sql_type="Bool"]
    pub in_favourites: bool,
    #[sql_type="Integer"]
    pub price: i32,
    #[sql_type="Text"]
    pub location: String,
    #[sql_type="Integer"]
    pub brand_id: i32,
    #[sql_type="Text"]
    pub brand_name: String,
    #[sql_type="Integer"]
    pub size_id: i32,
    #[sql_type="Text"]
    pub size_name: String,
    #[sql_type="Integer"]
    pub type_id: i32,
    #[sql_type="Text"]
    pub type_name: String,
    #[sql_type="Array<Text>"]
    pub pictures: Vec<String>,
    #[sql_type="Timestamp"]
    pub create_datetime: NaiveDateTime,
}

impl ProductContext {
    pub fn get_by_id(pr_id: i32,my_id: Option<i32>, conn: &PgConnection)-> (ProductContext,Users) {
        
        let product = diesel::sql_query(include_str!("../../SQL/product.sql"))
            .bind::<Integer,_>(pr_id)
            .bind::<Nullable<Integer>,_>(my_id)
            .get_result::<ProductContext>(conn)
            .expect("Error getting product context\n");
        
        let users = Users::get_by_id(product.seller_id, conn);

        (product,users)

    }
}



#[derive(Serialize, Deserialize,QueryableByName,Debug)]
pub struct ProductCard {
    #[sql_type="Integer"]
    pub id: i32,
    #[sql_type="Text"]
    pub title: String,
    #[sql_type="Integer"]
    pub price: i32,
    #[sql_type="Bool"]
    pub is_in_favourites: bool,
    #[sql_type="Text"]
    pub category_name: String,
    #[sql_type="Text"]
    pub size_name: String,
    #[sql_type="Array<Text>"]
    pub pictures: Vec<String>,
    #[sql_type="Integer"]
    pub seller_id: i32,
    #[sql_type="Text"]
    pub profile_picture: String,
    #[sql_type="Text"]
    pub seller_uname: String,
    #[sql_type="BigInt"]
    pub seller_rating: i64,
}

#[derive(FromForm,Clone,Serialize,Deserialize,Queryable,Debug)]
pub struct SearchForm {
    pub search_string: Option<String>,
    pub prod_size_id: Option<i32>,
    pub product_state_id: Option<i32>,
    pub limit: i32,
    pub offset: i32,
    pub subcategory_id: Option<i32>,
    pub category_id: Option<i32>,
    pub prod_brand_id: Option<i32>,
    pub prod_type_id: Option<i32>,
    pub order_by: Option<String>,
    pub user_id: Option<i32>,
}

impl ProductCard {

    pub fn get_favourites(user_id: i32, conn: &PgConnection) -> Vec<ProductCard> {
        
        diesel::sql_query(include_str!("../../SQL/get_favourites.sql"))
        .bind::<Nullable<Integer>, _>(user_id)
        .load::<ProductCard>(conn)
        .expect("diesel database error ProductCard::get_most_viewed(conn: &PgConnection)")

    }


    pub fn get_most_viewed(limit: i32, user_id: Option<i32>, conn: &PgConnection) -> Vec<ProductCard> {

        diesel::sql_query(include_str!("../../SQL/select_top_views.sql"))
            .bind::<Nullable<Integer>, _>(user_id)
            .bind::<Nullable<Integer>, _>(limit)
            .load::<ProductCard>(conn)
            .expect("diesel database error ProductCard::get_most_viewed(conn: &PgConnection)")

    }

    pub fn get_recently_added(limit: i32, user_id: Option<i32>, conn: &PgConnection) -> Vec<ProductCard> {

        diesel::sql_query(include_str!("../../SQL/select_recently_added.sql"))
            .bind::<Nullable<Integer>, _>(user_id)
            .bind::<Integer, _>(limit)
            .load::<ProductCard>(conn)
            .expect("diesel database error ProductCard:::get_recently_added(user_id: Option<i32>, conn: &PgConnection))")


    }

    pub fn get_by_seller_popular_products(user_id: Option<i32>, conn: &PgConnection) -> Vec<ProductCard> {
       
        diesel::sql_query(include_str!("../../SQL/select_popular_by_seller.sql"))
            .bind::<Nullable<Integer>, _>(user_id)
            .load::<ProductCard>(conn)
            .expect("error in executing product card get by seller")

    }

    pub fn filter_search(form: SearchForm, conn: &PgConnection) -> Vec<ProductCard> {
    
        diesel::sql_query(include_str!("../../SQL/filter.sql"))
            .bind::<Nullable<Text>,_>(form.search_string)
            .bind::<Nullable<Integer>,_>(form.prod_size_id)
            .bind::<Nullable<Integer>,_>(form.product_state_id)
            .bind::<Integer,_>(form.limit)
            .bind::<Integer,_>(form.offset)
            .bind::<Nullable<Integer>,_>(form.subcategory_id)
            .bind::<Nullable<Integer>,_>(form.category_id)
            .bind::<Nullable<Integer>,_>(form.prod_brand_id)
            .bind::<Nullable<Integer>,_>(form.prod_type_id)
            .bind::<Nullable<Text>,_>(form.order_by)
            .bind::<Nullable<Integer>,_>(form.user_id)
            .load::<ProductCard>(conn)
            .expect("Error executing filter method\n")
    }
    
}

// product additional stuff
#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductTodayViews { 
    pub id: i32,
    pub product_id: i32,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductCategories {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct FavouriteProducts {
    id: i32,
    user_id: i32,
    product_id: i32,
}

impl FavouriteProducts {
    pub fn add_favourite(u_id: i32, p_id: i32, conn: &PgConnection) -> Result<(),Error> {
        
        use rocket::http::Status;
        use crate::schema::favourites::dsl::*;
        if favourites
            .filter(user_id.eq(u_id))
            .filter(product_id.eq(p_id))
            .get_results::<FavouriteProducts>(conn)?
            .len() > 0 {
            return Err(Error {
                status: Status::InternalServerError,
                message: "product already exists in favourites".into(),

            });
        }
        diesel::insert_into(favourites)
        .values(&(user_id.eq(u_id),product_id.eq(p_id)))
        .execute(conn)?;
        Ok(())
    }

    pub fn delete_favourite(u_id: i32, p_id: i32, conn: &PgConnection) -> Result<(),Error> {
        
        use rocket::http::Status;
        use crate::schema::favourites::dsl::*;

        if favourites
            .filter(user_id.eq(u_id))
            .filter(product_id.eq(p_id))
            .get_results::<FavouriteProducts>(conn)?
            .len() == 0 {
            return Err(Error {
                status: Status::InternalServerError,
                message: "there is on such product in favourites to delete".into(),
            });
        }
        diesel::delete(favourites
            .filter(user_id.eq(u_id))
            .filter(product_id.eq(p_id)))
        .execute(conn)?;
        Ok(())

    }

}

#[derive(Insertable,AsChangeset,Debug,Clone)]
#[table_name="favourites"]
pub struct NewFavourites {
    user_id: i32,
    product_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductState {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductType {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Links {
    id: i32,
    name: String,
    link: String,
    icon: String,
}

impl Links {
    pub fn get_links(conn: &PgConnection) -> Result<Vec<Links>,Error> {

        use crate::schema::social_links::dsl::*;

        Ok(social_links
            .get_results::<Links>(conn)?)

    }
    pub fn change_link_by_id(l_id: i32, new_link: String, conn: &PgConnection) -> Result<(),Error> {

        use crate::schema::social_links::dsl::*;

        diesel::update(social_links)
            .filter(id.eq(l_id))
            .set(link.eq(new_link))
            .execute(conn)?;

        Ok(())

    }
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductSize {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductPromotions {
    id: i32,
    product_id: i32,
    is_marked: bool,
    top_by_cat: bool,
    top_by_name: bool,
    is_pre_order: bool,
    prod_bought_date: chrono::NaiveDateTime,
    in_news: bool,
}

impl ProductPromotions {
    pub fn get_pre_order(p_id: i32, conn: &PgConnection) -> Result<bool,Error> {

        use crate::schema::promotions::dsl::*;

        let p = promotions
            .filter(product_id.eq(p_id))
            .get_result::<ProductPromotions>(conn);
        match p {
            Err(_) => Ok(false),
            Ok(i) => Ok(i.is_pre_order),
        }

    }
}

#[derive(Insertable,AsChangeset,Debug,Clone)]
#[table_name="promotions"]
pub struct NewPromotion {
    product_id: i32,
    is_marked: bool,
    top_by_cat: bool,
    top_by_name: bool,
    is_pre_prder: bool,
}

#[derive(Serialize, Deserialize, QueryableByName, Clone)]
pub struct ProductRating {
    #[sql_type="Integer"]
    id: i32,
    #[sql_type="Integer"]
    voter_id: i32,
    #[sql_type="Text"]
    voter_photo: String,
    #[sql_type="Text"]
    voter_username: String,
    #[sql_type="Integer"]
    seller_id: i32,
    #[sql_type="SmallInt"]
    stars: i16,
    #[sql_type="Text"]
    comment: String,
    #[sql_type="Text"]
    feedback_type: String,
    #[sql_type="Timestamp"]
    create_datetime: chrono::NaiveDateTime,
}

impl ProductRating {
    pub fn set_rating(fid: i32, tid: i32, p_id: i32, stars_count: i16, com: String, feedb: String, conn: &PgConnection) {

        use crate::schema::rating::dsl::*;
        if fid == tid {
            return;
        }
        if diesel::select(diesel::dsl::exists(rating
            .filter(voter_id.eq(fid))
            .filter(seller_id.eq(tid))))
            .get_result(conn)
            .expect("err")
             {
                return;
            }
        diesel::insert_into(rating)
            .values((
                voter_id.eq(fid),
                seller_id.eq(tid),
                stars.eq(stars_count),
                comment.eq(com),
                product_id.eq(p_id),
                feedback_type.eq(feedb)))
            .execute(conn)
            .expect("Error adding rating to database");

    }

    pub fn get_by_user(u_id: i32, conn: &PgConnection) -> Vec<ProductRating> {

        diesel::sql_query(include_str!("../../SQL/rating.sql"))
            .bind::<Integer, _>(u_id)
            .load::<ProductRating>(conn)
            .expect("Error getting rating")

    }

}


#[derive(Insertable,AsChangeset,Debug,Clone)]
#[table_name="rating"]
pub struct NewroductRating {
    voter_id: i32,
    seller_id: i32,
    stars: i16,
    comment: String,
}



#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct Product {
    pub id: i32,
    pub sub_category_id: i32,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub product_state: i32,
    pub brand_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
    pub type_id: i32,
    pub size_id: i32,
    pub total_views: i64,
    pub create_datetime: NaiveDateTime,
    pub phone_number: String,
    pub status: String,
    pub bought_with: Option<i32>,
}

use rocket_contrib::templates::tera::Context;

#[derive(Clone, Debug, Serialize)]
pub struct ProductWithFav {
    prod: Product,
    favourites: usize,
    days_left: i64,
}

impl Product {
    pub fn get_context(&self) -> Context {
        let mut context = Context::new();
        context.insert("id", &self);
        context
    }
    pub fn get_by_id(pr_id: i32, conn: &PgConnection) -> Product {
        
        use crate::schema::products::dsl::*;

        products
            .filter(id.eq(pr_id))
            .get_result::<Product>(conn)
            .expect("error loading product")

    }

    pub fn get_for_profile(u_id: i32, conn: &PgConnection) -> Result<Vec<(i32,String)>,Error> {

        use crate::schema::users;

        Ok(products::table
            .filter(products::seller_id.eq(u_id))
            .filter(products::status.eq("published").or(products::status.eq("sold")))
            .inner_join(users::table.on(products::seller_id.eq(users::id)))
            .select((products::id,products::title))
            .get_results::<(i32,String)>(conn)?)

    }

    pub fn set_status(p_id: i32, stat: String, conn: &PgConnection) -> Result<(),Error> {

        use crate::schema::products::dsl::*;

        diesel::update(products.filter(id.eq(p_id)))
            .set(status.eq(stat))
            .execute(conn)?;
        Ok(())
    }

    pub fn set_customer_id(p_id: i32, c_id: i32, conn: &PgConnection) -> Result<(),Error> {

        use crate::schema::products::dsl::*;
        
        diesel::update(products.filter(id.eq(p_id)))
            .set(bought_with.eq(c_id))
            .execute(conn)?;
        Ok(())
    }

    pub fn get_products_by_status_and_user(stat: String, u_id: i32, conn: &PgConnection) -> Result<Vec<ProductWithFav>,Error> {

        use crate::schema::products::dsl::*;
        use crate::schema::favourites::dsl::*;

        use chrono::Duration;

        let r = products
            .filter(seller_id.eq(u_id))
            .filter(status.eq(stat))
            .get_results::<Product>(conn)?;

        let r = r.into_iter().map( | p | {
            ProductWithFav {
                favourites: favourites
                .filter(product_id.eq(p.id))
                .get_results::<FavouriteProducts>(conn)
                .expect("Error getting product favourites")
                .len(),
                days_left: -(chrono::Local::now().naive_utc()
                    .signed_duration_since(p.create_datetime + Duration::days(60))
                    .num_days()),
                prod: p,
            }
        }).collect();
        
        Ok(r)

    }

}


#[derive(Insertable,AsChangeset,Debug,Clone)]
#[table_name="products"]
pub struct NewProduct {
    pub sub_category_id: i32,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub product_state: i32,
    pub brand_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
    pub type_id: i32,
    pub size_id: i32,
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct Brand {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct SubCategory {
    id: i32,
    category_id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubCat {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AllSubCategories {
    pub category_name: String,
    pub category_id: i32,
    pub sub_categories: Vec<SubCat>,
}


#[derive(Serialize, Deserialize, QueryableByName, Clone)]
pub struct JoinedCategory {
    #[sql_type = "Text"]
    pub category_name: String,
    #[sql_type = "Text"]
    pub sub_category_name: String,
    #[sql_type = "Integer"]
    pub category_id: i32,
    #[sql_type = "Integer"]
    pub sub_category_id: i32,
}

#[derive(Serialize, Deserialize,QueryableByName)]
pub struct ProductAdmin {
    #[sql_type="Integer"]
    pub id: i32,
    #[sql_type="Text"]
    pub title: String,
    #[sql_type="Array<Text>"]
    pub pictures: Vec<String>,
    #[sql_type="Integer"]
    pub seller_id: i32,
    #[sql_type="Text"]
    pub seller_username: String,
    #[sql_type="Text"]
    pub seller_picture: String,
    #[sql_type="Bool"]
    pub is_published: bool,
}

use rocket::Data;
use rocket::http::ContentType;

impl ProductAdmin {
    pub fn get_products(page: i32, conn: &PgConnection) -> Vec<ProductAdmin> {
        diesel::sql_query(include_str!("../../SQL/get_products_admin.sql"))
            .bind::<Integer, _>((page-1)*12)
            .bind::<Integer, _>(12)
            .get_results::<ProductAdmin>(conn)
            .expect("error getting products")

    }

    pub fn publish(pr_id: i32, conn: &PgConnection) {
        use crate::schema::products::dsl::*;

        diesel::update(products)
            .filter(id.eq(pr_id))
            .set(status.eq("published"))
            .execute(conn)
            .expect("error setting published");

    }

    pub fn unpublish(pr_id: i32, conn: &PgConnection) {
        use crate::schema::products::dsl::*;

        diesel::update(products)
            .filter(id.eq(pr_id))
            .set(status.eq("refused"))
            .execute(conn)
            .expect("error setting published");

    }

    pub fn change_product(pr_id: i32, content_type: &ContentType, form: Data, conn: &PgConnection) {
        
        use crate::routes::product::parse_multiform_product;
        use crate::schema::products::dsl::*;
        use std::fs::remove_file;

        let p = parse_multiform_product(content_type, form);

        let prod = products
            .filter(id.eq(pr_id))
            .get_result::<Product>(conn)
            .expect("error getting product by id");
        
        prod.pictures.into_iter().for_each(
            | path | {
                remove_file(path).unwrap();
            }
        );
       
        diesel::update(products)
            .filter(id.eq(pr_id))
            .set(&p)
            .execute(conn)
            .expect("error setting published");
            
    }
}

use crate::routes::product::PrivForm;

impl PrivForm {

    pub fn save(&self, conn: &PgConnection) -> Result<(),Error> {
        use crate::schema::promotions::dsl::*;
        if self.all == false {
            diesel::insert_into(promotions)
                .values(
                    &(
                        product_id.eq(self.product_id),
                        is_marked.eq(false),
                        top_by_cat.eq(self.top_cat),
                        top_by_name.eq(self.top_name),
                        is_pre_prder.eq(self.pre_order),
                        in_news.eq(self.take_in_news),
                    ))
                .execute(conn)?;
        } else {
            diesel::insert_into(promotions)
            .values(
                &(
                    product_id.eq(self.product_id),
                    is_marked.eq(true),
                    top_by_cat.eq(true),
                    top_by_name.eq(true),
                    is_pre_prder.eq(self.pre_order),
                    in_news.eq(true),
                ))
            .execute(conn)?;
        }
        Ok(())
    }
}