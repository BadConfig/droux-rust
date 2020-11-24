use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::products::dsl::*;

use crate::models::product::*;

pub fn create_product(product: NewProduct, conn: &PgConnection) -> String {
    let res = diesel::insert_into(products)
        .values(&product)
        .returning(id)
        .get_results::<i32>(conn)
        .expect("error while adding product to database in create product");
    format!("{}",res[0])
}

pub fn get_brand_list(conn: &PgConnection) -> Vec<Brand> {
    use crate::schema::brands::dsl::*;
    brands
        .load::<Brand>(conn)
        .expect("error loading brands table")
}

pub fn reviewed_by_user(user_id: i32, seller: i32, conn: &PgConnection) -> bool {
    
    use crate::schema::rating::dsl::*;

    rating 
        .filter(voter_id.eq(user_id))
        .filter(seller_id.eq(seller))
        .count()
        .execute(conn)
        .expect("Err getting rate reviewed_by_user") > 0
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

pub fn increment_product_views(pr_id: i32, conn: &PgConnection) {
    diesel::update(products)
        .filter(id.eq(pr_id))
        .set(total_views.eq(total_views+1))
        .execute(conn)
        .expect("error incr views");
}

pub fn increment_product_today_views(pr_id: i32, conn: &PgConnection) {

    use crate::schema::views::dsl::*;

    let r = views
        .filter(product_id.eq(pr_id))
        .get_results::<ProductTodayViews>(conn)
        .expect("err incrementing today product");
    
    if r.len() > 0 {
        diesel::update(views)
            .filter(product_id.eq(pr_id))
            .set(count.eq(count+1))
            .execute(conn)
            .expect("error incr views");
    } else {
        diesel::insert_into(views)
            .values(&(product_id.eq(pr_id),count.eq(1)))
            .execute(conn)
            .expect("error incr views");
    }
}

pub fn get_category_list(conn: &PgConnection) -> Vec<AllSubCategories> {

    use diesel::sql_query;

    let query_data = sql_query("select
	categories.name as category_name,
	sub_categories.name as sub_category_name,
	categories.id as category_id,
	sub_categories.id as sub_category_id
    from categories join sub_categories 
    on sub_categories.category_id = categories.id;")
        .get_results::<JoinedCategory>(conn)
        .expect("error getting all data from result");

    let mut result = Vec::new();
    let mut prev_id = -1;
    for instance in query_data {
        if prev_id != instance.category_id {
            result.push(AllSubCategories {
                category_id: instance.category_id,
                category_name: instance.category_name,
                sub_categories: Vec::new()
            })
        }
        let last = result.len()-1;
        result
            .get_mut(last)
            .unwrap()
            .sub_categories
            .push( SubCat {
                id: instance.sub_category_id,
                name: instance.sub_category_name,
            });
        prev_id = instance.category_id;
    }
    result
}