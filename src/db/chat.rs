use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::chat_messages::dsl::*;


use crate::models::chat::{
    NewChat,
    Chat,
    NewChatMessage,
    ChatMessage,
    ChatPerson,
};

pub fn create_chat(my_id: i32, other_id: i32, prod_id: i32, conn: &PgConnection) -> Chat {
    use crate::schema::chat::dsl::*;

    let chat_data = NewChat {
        from_user_id: my_id,
        to_user_id: other_id,
        product_id: prod_id,
    };

    diesel::insert_into(chat)
        .values(&chat_data)
        .get_result(conn)
        .expect("Error while inserting to chat table in db::chat::get_chat")

}

pub fn chat_exists(my_id: i32, other_id: i32, prod_id: i32, conn: &PgConnection) -> bool {
    use crate::schema::chat::dsl::*;

    chat
        .filter(((
                from_user_id.eq(my_id)
                .and(to_user_id.eq(other_id))
            ) 
            .or(
                from_user_id.eq(other_id)
                .and(to_user_id.eq(my_id))
            )
        ).and(product_id.eq(prod_id)))
        .count()
        .get_result::<i64>(conn) != Err(diesel::NotFound)
        
}

pub fn get_chat(my_id: i32, other_id: i32, prod_id: i32, conn: &PgConnection) -> Chat {
    use crate::schema::chat::dsl::*;

    chat
        .filter(((
                from_user_id.eq(my_id)
                .and(to_user_id.eq(other_id))
            ) 
            .or(
                from_user_id.eq(other_id)
                .and(to_user_id.eq(my_id))
            )).and(product_id.eq(prod_id))
        )
        .get_result(conn)
        .expect("error getting chat with db::chat::get_chat")

}

pub fn get_messages(my_chat: Chat, my_id: i32, conn: &PgConnection) -> Vec<ChatMessage> {

    let result: Vec<ChatMessage> = chat_messages
        .filter(chat_id.eq(my_chat.id))
        .order(send_datetime.asc())
        .load(conn)
        .expect("error getting chat messages in get_messages");

    let id_vec: Vec<i32> = result.clone()
        .into_iter()
        .filter( | i | i.from_user_id != my_id)
        .map( | i | i.id )
        .collect();

    diesel::update(chat_messages)
        .filter(id.eq_any(id_vec))
        .set(is_read.eq(true))
        .execute(conn)
        .expect("error while setting is read flags db::chat::get_messages");

    result

}

pub fn having_unread(user_id: i32, conn: &PgConnection) -> bool {
    chat_messages
        .filter(from_user_id.eq(user_id).or(to_user_id.eq(user_id)).and(is_read.eq(false)))
        .count()
        .execute(conn)
        .expect("err reading chat msg") > 0
}

pub fn write_message(my_chat: Chat, my_id: i32, other_id: i32, message: String, conn: &PgConnection) {
    
    diesel::insert_into(chat_messages)
        .values(
            NewChatMessage {
                chat_id: my_chat.id,
                from_user_id: my_id,
                to_user_id: other_id,
                msg_data: message,
            }
        )
        .execute(conn)
        .expect("error adding chat to database");
}

pub fn get_dialoge_list(my_id: i32, conn: &PgConnection) -> Vec<ChatPerson> {
   
    use diesel::sql_query;
    use diesel::sql_types::Integer;

    sql_query("
    SELECT DISTINCT ON (users1.username) users1.username, users1.id, chat_messages.is_read, chat.product_id as product_id
    FROM chat_messages AS chat_messages INNER JOIN users AS users1
    ON chat_messages.from_user_id = users1.id 
    inner join chat on chat_messages.chat_id = chat.id WHERE (chat_messages.to_user_id = $1) 
    UNION   
    SELECT DISTINCT ON (users1.username) users1.username, users1.id, true, chat.product_id as product_id
    FROM chat_messages AS chat_messages INNER JOIN users AS users1
    ON chat_messages.to_user_id = users1.id  
    inner join chat on chat_messages.chat_id = chat.id WHERE (chat_messages.from_user_id = $1)")
    .bind::<Integer,_>(my_id)
    .get_results::<ChatPerson>(conn)
    .expect("Something went wrong while executing sql")

}