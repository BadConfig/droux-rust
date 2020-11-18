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

#[get("/admin/priveleges")]
pub fn admin_priveleges(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if !admin.is_admin {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    ctx.insert("priveleged",&Priveleges::get(&conn)?);
    Ok(Either::Template(Template::render("admin/priveleges", &ctx)))
}
 


#[derive(FromForm,Clone)]
pub struct PrivForm {
    user_id: i32,
    pr_type: String,
}

#[post("/admin/priveleges/delete",data="<form>")]
pub fn admin_priveleges_delete(form: Form<PrivForm>, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if !admin.is_admin {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Priveleges::delete(form.user_id, &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/priveleges")))
}

#[post("/admin/priveleges/change",data="<form>")]
pub fn admin_priveleges_change(form: Form<PrivForm>, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if !admin.is_admin {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Priveleges::change(form.user_id, form.pr_type.clone(), &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/priveleges")))
}

#[derive(FromForm,Clone)]
pub struct NewPrivForm {
    username: String,
    pr_type: String,
}

#[post("/admin/priveleges/add",data="<form>")]
pub fn admin_priveleges_add(form: Form<NewPrivForm>, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if !admin.is_admin {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Priveleges::insert(form.username.clone(), form.pr_type.clone(), &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/priveleges")))
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