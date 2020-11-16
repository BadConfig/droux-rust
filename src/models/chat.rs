extern crate serde;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use crate::schema::{chat,chat_messages};

#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct ChatMessage {
    pub id: i32,
    pub chat_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub msg_data: String,
    pub is_read: bool,
    pub send_datetime: NaiveDateTime,
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
    Integer, Bool, Text, Array, Timestamp, Nullable,
};
#[derive(Serialize, Deserialize, QueryableByName, Clone)]
pub struct ChatPerson {
    #[sql_type = "Integer"]
    pub chat_id: i32,
    #[sql_type = "Integer"]
    pub talker_id: i32,
    #[sql_type = "Text"]
    pub talker_username: String,
    #[sql_type = "Text"]
    pub talker_picture: String,
    #[sql_type = "Bool"]
    pub is_read: bool,
    #[sql_type = "Integer"]
    pub pr_id: i32,
    #[sql_type = "Text"]
    pub pr_title: String,
    #[sql_type = "Array<Text>"]
    pub pr_pictures: Vec<String>,
    #[sql_type = "Integer"]
    pub pr_price: i32,
    #[sql_type = "Nullable<Timestamp>"]
    pub last_msg_time: Option<chrono::NaiveDateTime>,
    #[sql_type = "Nullable<Text>"]
    pub last_msg: Option<String>,
}