use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use crate::routes::Either;
use crate::db::filters::get_filter_context;
use crate::models::product::{
    ProductAdmin,
    Product,
};
use super::get_base_context;
use crate::users::CommonUser;



//routes for admin
#[get("/admin")]
pub fn admin_main(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    Template::render("admin/main", &ctx)
}

#[get("/admin/users/<page>")]
pub fn admin_product(page: i32, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        return Either::Redirect(Redirect::to("/admin"))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    ctx.insert("current_page",&page);
    ctx.insert("products", &ProductAdmin::get_products(page, &conn));
    Either::Template(Template::render("admin/products", &ctx))
}

// #[get("/admin/product/<page>")]
// pub fn admin_product(page: i32, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
//     if admin.is_editor {
//         print!("true\n");
//         return Either::Redirect(Redirect::to("/admin"))
//     }
//     let mut ctx = get_base_context(user, &conn);
//     ctx.insert("admin",&admin);
//     ctx.insert("current_page",&page);
//     ctx.insert("products", &ProductAdmin::get_products(page, &conn));
//     Either::Template(Template::render("admin/products", &ctx))
// }

// #[post("/admin/user/ban")]
// pub fn admin_main(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    
// }

#[get("/admin/product/edit/<id>")]
pub fn admin_product_edit(id: i32, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        return Either::Redirect(Redirect::to("/admin"))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    get_filter_context(&mut ctx, &conn);
    ctx.insert("prod_data", &Product::get_by_id(id, &conn));
    Either::Template(Template::render("admin/edit_products", &ctx))
}

#[post("/admin/product/refuse/<page>/<id>")]
pub fn admin_product_unpub(page: i32, id: i32, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        return Either::Redirect(Redirect::to("/admin"))
    }
    ProductAdmin::unpublish(id, &conn);
    Either::Redirect(Redirect::to(format!("/admin/product/{}",&page)))
}

#[post("/admin/product/refuse/<page>/<id>")]
pub fn admin_product_change(page: i32, id: i32, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        return Either::Redirect(Redirect::to("/admin"))
    }
    ProductAdmin::unpublish(id, &conn);
    Either::Redirect(Redirect::to(format!("/admin/product/{}",&page)))
}
use rocket::Data;
use rocket::http::ContentType;

#[post("/admin/product/change/<id>",data="<form>")]
pub fn product_change(id: i32, content_type: &ContentType, form: Data, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Redirect {
    if admin.is_editor {
        return Redirect::to("/admin")
    }
    ProductAdmin::change_product(id, content_type, form, &conn);
    ProductAdmin::publish(id, &conn);
    Redirect::to("/admin/product/0")
}