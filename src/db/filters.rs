use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::products::dsl::*;
use rocket_contrib::templates::tera::Context;
use serde::{
    Serialize,
    Deserialize,
};

use crate::models::product::*;

pub fn get_filter_context(ctx: &mut Context, conn: &PgConnection) {
    
    use crate::db::product::get_category_list;

    ctx.insert("ProductBrands", &get_brands(conn));
    ctx.insert("ProductSizes",&get_product_sizes(conn));
    ctx.insert("ProductTypes", &get_product_types(conn));
    ctx.insert("ProductStates", &get_product_states(conn));
    ctx.insert("ProductCategories", &get_category_list(conn));

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

pub fn get_brands(conn: &PgConnection) ->  Vec<Brand> {

    use crate::schema::brands::dsl::*;

    brands
        .load::<Brand>(conn)
        .expect("Error loading all brands")
}

pub fn get_product_types(conn: &PgConnection) -> Vec<ProductType> {
    use crate::schema::product_type::dsl::*;

    product_type
        .load::<ProductType>(conn)
        .expect("error loading types\n")
}

pub fn get_product_sizes(conn: &PgConnection) -> Vec<ProductSize> {
    
    use crate::schema::sizes::dsl::*;

    sizes
        .load::<ProductSize>(conn)
        .expect("error loading size\n")
}

pub fn get_product_states(conn: &PgConnection) -> Vec<ProductState> {

    use crate::schema::product_state::dsl::*;

    product_state
        .load::<ProductState>(conn)
        .expect("Error loading product states")

}



