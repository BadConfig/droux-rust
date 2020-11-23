use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use crate::routes::Either;

use super::get_base_context;
use crate::users::CommonUser;
use crate::models::product::ProductCard;

use crate::auth::make_jwt_for_user;

#[get("/login")]
pub fn login(user: CommonUser, conn: crate::db::Conn) -> Template {
    let mut ctx = get_base_context(user, &conn);
    ctx.insert("auth_fail",&false);
    Template::render("auth/login", &ctx)
}

#[derive(FromForm,Clone)]
pub struct LoginForm {
    login: String,
    pass: String,
}
#[post("/login", data = "<form>")]
pub fn authorize(form: Form<LoginForm>, user: CommonUser, mut cookies: Cookies, conn: crate::db::Conn) -> Either {

    use crate::auth::Login;
    use crate::db::users::auth_user;
    
    if let CommonUser::Logged(_) = user {
        return Either::Redirect(Redirect::to("/"));
    }

    let login = Login::new(form.login.clone());
    if let Ok(user) = auth_user(login, form.pass.clone(), &conn) {
        cookies.add_private(Cookie::new("user",make_jwt_for_user(
            user.email.clone(),
            user.username.clone(),
            form.pass.clone())));
        Either::Redirect(Redirect::to("/"))
    } else {
        print!("wrong user data");
        let mut ctx = get_base_context(user.clone(), &conn);
        ctx.insert("login_fail",&true);
        ctx.insert("register_fail",&false);
        let opt_id = match user.clone() {
            CommonUser::Logged(u) => Some(u.id),
            CommonUser::NotLogged() => None,
        };
        ctx.insert("most_viewed_products",&ProductCard::get_most_viewed(20,opt_id.clone(), &conn));
        ctx.insert("new_products", &ProductCard::get_recently_added(20,opt_id.clone(), &conn));
        ctx.insert("popular_seller_products", &ProductCard::get_by_seller_popular_products(opt_id.clone(), &conn));
        Either::Template(Template::render("index", &ctx))
    }

}

#[get("/logout")]
pub fn logout(user: CommonUser, mut cookies: Cookies) -> Redirect {
    if let CommonUser::NotLogged() = user {
        return Redirect::to("/");
    }
    cookies.add_private(Cookie::new("user"," "));
    Redirect::to("/")
}

#[derive(FromForm,Clone)]
pub struct RegisterForm {
    username: String,
    email: String,
    pass: String,
}
#[post("/register",data="<form>")]
pub fn register(form: Form<RegisterForm>, user: CommonUser, conn: crate::db::Conn) -> Either {
   
    use crate::db::users::create_user;
    let opt_id = match user.clone() {
        CommonUser::Logged(u) => Some(u.id),
        CommonUser::NotLogged() => None,
    };
    let mut ctx = get_base_context(user.clone(), &conn);
    ctx.insert("most_viewed_products",&ProductCard::get_most_viewed(20,opt_id.clone(), &conn));
    ctx.insert("new_products", &ProductCard::get_recently_added(20,opt_id.clone(), &conn));
    ctx.insert("popular_seller_products", &ProductCard::get_by_seller_popular_products(opt_id.clone(), &conn));
    let link = create_user(form.username.clone(), form.email.clone(), form.pass.clone(), &conn);
    let link = match link {
        Ok(s) => s,
        Err(e) => {
            print!("error getting link in register POST func");
            ctx.insert("login_fail",&false);
            ctx.insert("register_fail",&true);
            ctx.insert("err_field", &e[..]);
            return Either::Template(Template::render("index", &ctx));
        },
    };
    let url = "/verify/".to_string()+&link[..];
    print!("url: {}\n",&url);
    Either::Redirect(Redirect::to(url))
    //print!("auth ref: {}",reference);
    //crate::auth::send_auth_link(reference, form.email.clone());
    //Template::render("auth/verify",get_base_context(user, &conn))

}

#[get("/register")]
pub fn register_get(user: CommonUser, conn: crate::db::Conn) -> Template {
    Template::render("auth/register", get_base_context(user, &conn))
}

#[get("/verify/<link>")]
pub fn verify_link(link: String, conn: crate::db::Conn) -> Redirect {
   
    use crate::db::auth::activate_user;

    match activate_user(link, &conn) {
        Ok(()) => Redirect::to("/"),
        Err(()) => Redirect::to("/"),
    }

}

