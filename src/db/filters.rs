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



