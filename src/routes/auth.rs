use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};

use rocket::State;
use crate::auth::IsLogged;
use std::sync::atomic::{Ordering,AtomicUsize};
use super::get_base_context;
use crate::users::CommonUser;

use crate::auth::make_jwt_for_user;
use crate::routes::Either;

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
        let mut ctx = get_base_context(user, &conn);
        ctx.insert("auth_fail",&true);
        Either::Template(Template::render("auth/login", &ctx))
    }

}

#[get("/logout")]
pub fn logout(user: CommonUser, mut cookies: Cookies, conn: crate::db::Conn) -> Redirect {
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
pub fn register(form: Form<RegisterForm>, user: CommonUser, conn: crate::db::Conn) -> Template {
   
    use crate::db::users::create_user;

    let link = create_user(form.username.clone(), form.email.clone(), form.pass.clone(), &conn);
    let link = match link {
        Ok(s) => s,
        Err(e) => {
            print!("error getting link in register POST func");
            let mut ctx = get_base_context(user, &conn);
            ctx.insert("reg_sucsess",&false);
            ctx.insert("err_field", &e[..]);
            return Template::render("auth/register", &ctx)
        },
    };
    let reference = "localhost:8000".to_string() + "/verify/" + &link[..];
    print!("auth ref: {}",reference);
    crate::auth::send_auth_link(reference, form.email.clone());
    Template::render("auth/verify",get_base_context(user, &conn))

}

#[get("/register")]
pub fn register_get(user: CommonUser, conn: crate::db::Conn) -> Template {
    Template::render("auth/register", get_base_context(user, &conn))
}

#[get("/verify/<link>")]
pub fn verify_link(link: String, conn: crate::db::Conn) -> Redirect {
   
    use crate::db::auth::activate_user;

    match activate_user(link, &conn) {
        Ok(()) => Redirect::to("/login"),
        Err(()) => Redirect::to("/"),
    }

}

