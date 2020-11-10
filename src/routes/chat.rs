use rocket_contrib::templates::{Template,tera::*};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};

use rocket::State;
use crate::auth::IsLogged;
use std::sync::atomic::{Ordering,AtomicUsize};
use super::get_base_context;
use crate::users::CommonUser;
use crate::db::chat::*;

use crate::auth::make_jwt_for_user;
use crate::routes::Either;
use crate::models::users::Users;

#[get("/chat/<prod_id>/<user_id>")]
pub fn get_chat_messages(user: CommonUser, user_id: i32, prod_id: i32, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        let chat = get_chat(user.id, user_id, prod_id, &conn);

        ctx.insert("people_list",&get_dialoge_list(user.id, &conn));
        ctx.insert("messages",&get_messages(chat, user.id, &conn));
        ctx.insert("from_user",&Users::get_by_id(user_id, &conn));
        ctx.insert("prod_id",&prod_id);
        Either::Template(Template::render("users/chat_dialogue", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
}

#[get("/chat")]
pub fn get_chats(user: CommonUser, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        ctx.insert("people_list",&get_dialoge_list(user.id, &conn));
        Either::Template(Template::render("users/chat_content", &ctx))
    } else {
        Either::Redirect(Redirect::to("/"))
    }
} 
#[derive(FromForm,Clone)]
pub struct CreateChatForm {
    other_id: i32,
    product_id: i32,
}
#[post("/chat/create",data="<form>")]
pub fn create_chat_messages(user: CommonUser, form: Form<CreateChatForm>, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        print!("my_id: {} other_id: {} prod_id: {}",user.id,form.other_id,form.product_id);
        if form.other_id == user.id {
            return Either::Redirect(Redirect::to("/")); // lkkk
        }
        if chat_exists(user.id, form.other_id, form.product_id, &conn) {
            return Either::Redirect(Redirect::to(format!(
                "/chat/{}/{}",
                form.product_id,
                form.other_id)));
        }


        let chat = create_chat(user.id, form.other_id, form.product_id, &conn);
        Either::Redirect(Redirect::to(format!(
            "/chat/{}/{}",
            chat.product_id,
            chat.to_user_id)))
    } else {
        Either::Redirect(Redirect::to("/login"))

    }
}

#[derive(FromForm,Clone)]
pub struct MessageForm {
    other_id: i32,
    product_id: i32,
    message: String,
}
#[post("/chat/send",data="<form>")]
pub fn write_chat_messages(user: CommonUser,form: Form<MessageForm>, conn: crate::db::Conn) -> Either {
    let mut ctx = get_base_context(user.clone(), &conn);
    if let CommonUser::Logged(user) = user {
        let chat = get_chat(
            user.id,
                form.other_id,
                form.product_id,
                &conn);
        write_message(chat,
                user.id,
                form.other_id,
                form.message.clone(),
                &conn);
        Either::Redirect(Redirect::to(format!(
            "/chat/{}/{}",
            form.product_id,
            form.other_id,)))
    } else {
        Either::Redirect(Redirect::to("/login"))

    }
}

