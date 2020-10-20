use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;

//routes for admin
#[get("/admin")]
pub fn admin_main(admin: crate::admin::AdminUser) -> Template {
    let mut ctx = Context::new();
    ctx.insert("is_admin",&admin.is_admin);
    ctx.insert("is_moderator",&admin.is_moderator);
    ctx.insert("is_editor", &admin.is_editor);
    Template::render("admin_pannel", &ctx)
}

