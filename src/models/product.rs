use chrono::NaiveDateTime;
extern crate serde;
use serde::{Serialize, Deserialize};
use crate::schema::{
    products,
    categories,
    sub_categories,
};
use rocket_contrib::templates::tera::*;

#[derive(Serialize, Deserialize)]
pub struct ProductContext {
    pub id: i32,
    pub sub_category_id: i32,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub state: String,
    pub brand_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
    pub create_datetime: NaiveDateTime,
    pub category_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Product {
    pub id: i32,
    pub sub_category_id: i32,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub state: String,
    pub brand_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
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
    pub state: String,
    pub brand_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
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

use diesel::sql_types::{
    Integer, Text,
};
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