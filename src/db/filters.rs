use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::products::dsl::*;
use rocket_contrib::templates::tera::Context;
use serde::{
    Serialize,
    Deserialize,
    
};
use crate::db::product::get_category_list;
use crate::models::product::*;

pub fn get_filter_context(ctx: &mut Context, conn: &PgConnection) {


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

pub fn get_categories_for_header(conn: &PgConnection) -> Vec<AllSubCategories> {
    let ret = get_category_list(conn);
    for i in ret {
        i.sub_categories.sort();
    }
    ret
}

pub fn get_brands_for_header(conn: &PgConnection) -> Vec<Vec<Brand>> {
    let res = Vec::new();
    let br = get_brands(conn);
    br.sort();
    let fl = br[0].name.chars().nth(0).unwrap();
    for i in br {
        if fl != i.name.chars().nth(0).unwrap() {
            res.push(Vec::new());
        }
        res[res.len()-1].push(i);
    }
    print!("\n{:?}\n",&res);
    res
}
