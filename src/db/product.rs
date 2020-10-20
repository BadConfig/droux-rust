use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::products::dsl::*;

use crate::models::product::*;

pub fn create_product(product: NewProduct, conn: &PgConnection) {
    diesel::insert_into(products)
        .values(&product)
        .execute(conn)
        .expect("error while adding product to database in create product");
}

pub fn get_brand_list(conn: &PgConnection) -> Vec<Brand> {
    use crate::schema::brands::dsl::*;
    brands
        .load::<Brand>(conn)
        .expect("error loading brands table")
}

pub fn get_product_data(pr_id: i32, conn: &PgConnection) -> Product {
    products
        .filter(id.eq(pr_id))
        .get_result::<Product>(conn)
        .expect("no such product found")
}

pub fn get_brand_name(cid: i32, conn: &PgConnection) -> Brand {
    use crate::schema::brands::dsl::*;
    brands
        .filter(id.eq(cid))
        .get_result::<Brand>(conn)
        .expect("no such brand found by id")
}