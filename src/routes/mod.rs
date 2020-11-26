pub mod admin;
pub mod auth;
pub mod product;
pub mod filters;
pub mod users;
pub mod chat;
pub mod static_pages;

use rocket_contrib::templates::{Template,tera::*};
use rocket::response::Redirect;
use diesel::PgConnection;

use crate::models::product::ProductCard;

use crate::users::CommonUser;

#[get("/")]
pub fn index(user: CommonUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user.clone(), &conn);
    let opt_id = match user.clone() {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    ctx.insert("most_viewed_products",&ProductCard::get_most_viewed(20,opt_id.clone(), &conn));
    ctx.insert("new_products", &ProductCard::get_recently_added(20,opt_id.clone(), &conn));
    ctx.insert("popular_seller_products", &ProductCard::get_by_seller_popular_products(opt_id.clone(), &conn));
    Template::render("index", &ctx )
}

use crate::models::users::Users;
pub enum UserGuard {
    Logged(Users),
    NotLogged,
}
pub fn get_required_context(data: UserGuard, conn: &PgConnection) -> Context {
    let mut ctx = Context::new();
    use UserGuard::*;
    use crate::db::filters::{
        get_categories_for_header,
    };
    ctx.insert("header_brands", &get_brands_for_header(&conn));
    ctx.insert("header_categories", &get_categories_for_header(&conn));
    ctx.insert("login_fail",&false);
    ctx.insert("register_fail",&false);
    ctx.insert("links", &crate::models::admin::Links::get_social_media_links(conn));
    match data {
        Logged(usr) => {
            ctx.insert("is_logged", &true);
            ctx.insert("user", &usr);
        },
        NotLogged => ctx.insert("is_logged", &false),
    }

    ctx
}


pub fn get_base_context(usr: CommonUser, conn: &PgConnection) -> Context {

    use UserGuard::*;

    let usr_data = if let CommonUser::Logged(user_data) = usr {
        Logged(user_data)
    } else {
        NotLogged
    };

    get_required_context(usr_data, conn)

}

#[derive(Debug, Responder)]
pub enum Either {
    Template(Template),
    Redirect(Redirect),
}
