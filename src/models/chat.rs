extern crate serde;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use crate::schema::{chat,chat_messages};

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct ChatMessage {
    pub id: i32,
    chat_id: i32,
    pub from_user_id: i32,
    to_user_id: i32,
    msg_data: String,
    is_read: bool,
    send_datetime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Chat {
    pub id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub product_id: i32,
}

#[derive(Insertable,AsChangeset)]
#[table_name="chat"]
pub struct NewChat {
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub product_id: i32,
}

#[derive(Insertable,AsChangeset)]
#[table_name="chat_messages"]
pub struct NewChatMessage {
    pub chat_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub msg_data: String,

}

use diesel::sql_types::{
    Integer, Bool, Text,
};
#[derive(Serialize, Deserialize, QueryableByName, Clone)]
pub struct ChatPerson {
    #[sql_type = "Text"]
    pub username: String,
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Bool"]
    pub is_read: bool,
    #[sql_type = "Integer"]
    pub product_id: i32,
}