use chrono::NaiveDateTime;
extern crate serde;
use serde::{Serialize, Deserialize};
use crate::schema::products;
use rocket_contrib::templates::tera::*;

#[derive(Serialize, Deserialize)]
pub struct ProductContext {
    pub id: i32,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub state: String,
    pub category_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
    pub create_datetime: NaiveDateTime,
    pub category_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub state: String,
    pub category_id: i32,
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
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub location: String,
    pub state: String,
    pub category_id: i32,
    pub seller_id: i32,
    pub pictures: Vec<String>,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Brand {
    pub id: i32,
    pub name: String,
}
