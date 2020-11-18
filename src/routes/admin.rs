use rocket_contrib::templates::{Template};
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
use crate::models::users::Users;
use crate::Error;


//routes for admin
#[get("/admin")]
pub fn admin_main(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    Template::render("admin/main", &ctx)
}

#[get("/admin/users/<page>")]
pub fn admin_users(page: i64, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    ctx.insert("current_page",&page);
    ctx.insert("users", &Users::get_with_page(page, &conn)?);
    Ok(Either::Template(Template::render("admin/users", &ctx)))
}

#[get("/admin/product/<page>")]
pub fn admin_product(page: i32, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        print!("true\n");
        return Either::Redirect(Redirect::to("/admin"))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    ctx.insert("current_page",&page);
    ctx.insert("products", &ProductAdmin::get_products(page, &conn));
    Either::Template(Template::render("admin/products", &ctx))
}

#[derive(FromForm,Clone)]
pub struct BanForm {
    user_id: i32,
    page: i32,
    change_flag: bool,
}

#[post("/admin/users/ban",data="<form>")]
pub fn admin_users_ban(form: Form<BanForm>, _user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Users::change_banned(form.change_flag, form.user_id, &conn)?;
    Ok(Either::Redirect(Redirect::to(format!("/admin/users/{}",&form.page))))
}



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
pub fn admin_product_unpub(page: i32, id: i32, _user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        return Either::Redirect(Redirect::to("/admin"))
    }
    ProductAdmin::unpublish(id, &conn);
    Either::Redirect(Redirect::to(format!("/admin/product/{}",&page)))
}

#[post("/admin/product/refuse/<page>/<id>")]
pub fn admin_product_change(page: i32, id: i32, _user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Either {
    if admin.is_editor {
        return Either::Redirect(Redirect::to("/admin"))
    }
    ProductAdmin::unpublish(id, &conn);
    Either::Redirect(Redirect::to(format!("/admin/product/{}",&page)))
}
use rocket::Data;
use rocket::http::ContentType;

#[post("/admin/product/change/<id>",data="<form>")]
pub fn product_change(id: i32, content_type: &ContentType, form: Data, _user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Redirect {
    if admin.is_editor {
        return Redirect::to("/admin")
    }
    ProductAdmin::change_product(id, content_type, form, &conn);
    ProductAdmin::publish(id, &conn);
    Redirect::to("/admin/product/0")
}