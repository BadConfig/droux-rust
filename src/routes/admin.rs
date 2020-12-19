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
use rocket::Data;
use rocket::http::ContentType;
use crate::users::CommonUser;
use crate::models::users::Users;
use crate::Error;
use crate::models::admin::Priveleges;
use crate::models::product::Links;
use crate::models::news::News;
use crate::models::rescue::Issue;

#[post("/admin/news/add",data="<form>")]
pub fn news_add(form: Data, content_type: &ContentType, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_moderator {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    News::create_news(content_type, form, &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/news")))
}

#[derive(FromForm,Clone)]
pub struct NewsForm {
    id: i32,
}
#[post("/admin/news/delete",data="<form>")]
pub fn news_delete(form: Form<NewsForm>, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_moderator {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    News::delete(form.id, &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/news")))
}

#[get("/admin/news/add")]
pub fn news_make(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_moderator {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    let mut ctx = get_base_context(user, &conn);
    Ok(Either::Template(Template::render("admin/news_add", ctx)))
}

#[get("/admin/news")]
pub fn news_show(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_moderator {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("news_list", &News::pages(&conn)?);
    Ok(Either::Template(Template::render("admin/news", ctx)))
}

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

#[get("/admin/links")]
pub fn admin_links(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("admin",&admin);
    ctx.insert("social_links", &Links::get_links(&conn)?);
    Ok(Either::Template(Template::render("admin/links", &ctx)))
}

#[derive(FromForm,Clone)]
pub struct LinkForm {
    link_id: i32,
    new_link: String,
}

#[post("/admin/links/change",data="<form>")]
pub fn admin_links_change(form: Form<LinkForm>, _user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Links::change_link_by_id(form.link_id, form.new_link.clone(), &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/links")))
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
pub fn admin_users_ban(form: Form<BanForm>, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Users::change_banned(form.change_flag, form.user_id, &conn)?;
    Ok(Either::Redirect(Redirect::to(format!("/admin/users/{}",&form.page))))
}

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
pub fn admin_priveleges_delete(form: Form<PrivForm>, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if !admin.is_admin {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Priveleges::delete(form.user_id, &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/priveleges")))
}

#[post("/admin/priveleges/change",data="<form>")]
pub fn admin_priveleges_change(form: Form<PrivForm>, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
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
pub fn admin_priveleges_add(form: Form<NewPrivForm>, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
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

#[post("/admin/product/change/<id>",data="<form>")]
pub fn product_change(id: i32, content_type: &ContentType, form: Data, _user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Redirect {
    if admin.is_editor {
        return Redirect::to("/admin")
    }
    ProductAdmin::change_product(id, content_type, form, &conn);
    ProductAdmin::publish(id, &conn);
    Redirect::to("/admin/product/0")
}

#[get("/admin/rescue/list")]
pub fn rescue_list(user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")));
    }
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("issues", &Issue::list(0, 100, &conn)?);
    Ok(Either::Template(Template::render("admin/rescue", ctx)))
}

#[derive(FromForm)]
pub struct RescueId {
    id: i32,
}

#[post("/admin/rescue/delete",data="<form>")]
pub fn rescue_delete(form: Form<RescueId>, user: CommonUser, admin: crate::admin::AdminUser, conn: crate::db::Conn) -> Result<Either,Error> {
    if admin.is_editor {
        return Ok(Either::Redirect(Redirect::to("/admin")))
    }
    Issue::delete(form.id, &conn)?;
    Ok(Either::Redirect(Redirect::to("/admin/rescue/list")))
}