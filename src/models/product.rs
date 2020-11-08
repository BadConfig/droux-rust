use chrono::NaiveDateTime;
extern crate serde;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    products,
    categories,
    sub_categories,
    favourites,
    promotions,
    rating,
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable
};
use rocket_contrib::templates::tera::*;
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

        use crate::models::users::Users;
        
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
    #[sql_type="Text"]
    pub price: String,
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
    search_string: Option<String>,
    prod_size_id: Option<i32>,
    product_state_id: Option<i32>,
    limit: i32,
    offset: i32,
    subcategory_id: Option<i32>,
    category_id: Option<i32>,
    prod_brand_id: Option<i32>,
    prod_type_id: Option<i32>,
    order_by: String,
    user_id: Option<i32>,
}

impl ProductCard {

    pub fn get_most_viewed(user_id: Option<i32>, conn: &PgConnection) -> Vec<ProductCard> {
        
        use diesel::sql_types::{
            Nullable,
            Integer,
        };

        diesel::sql_query(include_str!("../../SQL/select_top_views.sql"))
            .bind::<Nullable<Integer>, _>(user_id)
            .load::<ProductCard>(conn)
            .expect("diesel database error ProductCard::get_most_viewed(conn: &PgConnection)")

    }

    pub fn get_recently_added(user_id: Option<i32>, conn: &PgConnection) -> Vec<ProductCard> {
       
        use diesel::sql_types::{
            Nullable,
            Integer,
        };

        diesel::sql_query(include_str!("../../SQL/select_popular_by_seller.sql"))
            .bind::<Nullable<Integer>, _>(user_id)
            .load::<ProductCard>(conn)
            .expect("diesel database error ProductCard:::get_recently_added(user_id: Option<i32>, conn: &PgConnection))")


    }

    pub fn get_by_seller_popular_products(user_id: Option<i32>, conn: &PgConnection) -> Vec<ProductCard> {
       
        use diesel::sql_types::{
            Nullable,
            Integer,
        };
       
        diesel::sql_query(include_str!("../../SQL/select_popular_by_seller.sql"))
            .bind::<Nullable<Integer>, _>(user_id)
            .load::<ProductCard>(conn)
            .expect("error in executing product card get by seller")

    }

    pub fn filter_search(form: SearchForm, conn: &PgConnection) -> Vec<ProductCard> {

        use diesel::sql_types::{
            Integer,
            Text,
            Nullable,
        };
    
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
            .bind::<Text,_>(form.order_by)
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
pub struct DeletedProducts {
    id: i32,
    post_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct FavouriteProducts {
    id: i32,
    user_id: i32,
    product_id: i32,
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

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ProductRating {
    id: i32,
    voter_id: i32,
    seller_id: i32,
    stars: i16,
    comment: String,
}

impl ProductRating {
    pub fn set_rating(fid: i32, tid: i32, stars_count: i16, conn: &PgConnection) {

        use crate::schema::rating::dsl::*;

        diesel::insert_into(rating)
            .values((voter_id.eq(fid),seller_id.eq(tid),stars.eq(stars_count)))
            .execute(conn)
            .expect("Error adding rating to database");

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



#[derive(Serialize, Deserialize, Queryable, Clone)]
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
    pub is_published: bool,
    pub type_id: i32,
    pub size_id: i32,
    pub total_views: i64,
    pub create_datetime: NaiveDateTime,
}

impl Product {
    pub fn get_context(&self) -> Context {
        let mut context = Context::new();
        context.insert("id", &self);
        context
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
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
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