use chrono::NaiveDateTime;
extern crate serde;
use crate::Error;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{
    news,
};
use diesel::sql_types::{
    Integer, Text, Bool, BigInt, Array, Timestamp, Nullable, SmallInt,
};

extern crate rocket_multipart_form_data;
use rocket::Data;
use rocket::http::ContentType;


#[derive(Serialize, Deserialize, Queryable, Clone, Debug)]
pub struct News{
    pub id: i32,
    pub title: String,
    pub body: String,
    pub picture: String,
    pub creation_datetime: NaiveDateTime,
    pub subtitle: String,
    pub banner: String,
}

#[derive(Insertable,AsChangeset,Debug,Clone)]
#[table_name="news"]
pub struct NewNews{
    pub title: String,
    pub body: String,
    pub picture: String,
    pub subtitle: String,
    pub banner: String,
}

pub fn parse_multiform_news(content_type: &ContentType, form: Data) -> Result<NewNews,Error> {
    
    use rocket_multipart_form_data::{
        MultipartFormData, 
        MultipartFormDataField, 
        MultipartFormDataOptions,
    };
    use std::fs::File;
    use std::io::prelude::*;

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::raw("picture")
                .size_limit(16*1920*1080),
            MultipartFormDataField::raw("banner")
                .size_limit(16*1920*1080),
            MultipartFormDataField::text("title"),
            MultipartFormDataField::text("subtitle"),
            MultipartFormDataField::text("body"),
        ]
    );

    fn unpack(data: Option<Vec<rocket_multipart_form_data::TextField>>) -> Result<String,Error> {
        Ok(data
            .ok_or("error getting M/F-D text field".to_string())?
            .remove(0)
            .text)
    }
    fn unpack_photo(photo: &Option<&Vec<rocket_multipart_form_data::RawField>>, title: &String) -> Result<String,Error> {
        let ph = photo
            .ok_or("error getting M/F-D photo".to_string())?
            .into_iter()
            .nth(0)
            .ok_or("error getting M/F-D photo".to_string())?
            .raw
            .clone();
        let store_path = format!("static/news/{}-{}.jpg",chrono::Local::now().naive_utc(),title);
        let mut file = File::create(&store_path)?;
        file.write_all(&ph).expect("error wfiting photo to file");
        Ok(store_path)
    }

    let mut multipart_form_data = MultipartFormData::parse(content_type, form, options).unwrap();

    let title       = unpack(multipart_form_data.texts.remove("title"))?;
    let picture     = unpack_photo(&multipart_form_data.raw.get("picture"), &(title.clone() + "_pic"))?; // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let banner      = unpack_photo(&multipart_form_data.raw.get("banner"), &(title.clone() + "_pic"))?;
    let body        = unpack(multipart_form_data.texts.remove("body"))?;
    let subtitle    = unpack(multipart_form_data.texts.remove("subtitle"))?;

    Ok(NewNews{
        title: title,
        picture: picture,
        banner: banner,
        subtitle: subtitle,
        body: body,
    })

}

impl News {

    pub fn create_news(content_type: &ContentType, form: Data, conn: &PgConnection) -> Result<(),Error> {   
        use crate::schema::news::dsl::*;

        let news_raw = parse_multiform_news(content_type, form)?;
        diesel::insert_into(news)
            .values(&news_raw)
            .execute(conn)?;

        Ok(())
    }

    pub fn banners(conn: &PgConnection) -> Result<Vec<News>,Error> {
        use crate::schema::news::dsl::*;

        Ok(news
            .order_by(creation_datetime)
            .limit(6)
            .get_results::<News>(conn)?)
    }

    pub fn delete(news_id: i32, conn: &PgConnection) -> Result<(),Error> {
        use crate::schema::news::dsl::*;
        use std::fs::remove_file;

        let row = news
            .filter(id.eq(news_id))
            .get_result::<News>(conn)?;
        
        remove_file(row.banner)?;
        remove_file(row.picture)?;
        
        diesel::delete(news
            .filter(id.eq(news_id)))
            .execute(conn)?;
        Ok(())
    }

    pub fn pages(conn: &PgConnection) -> Result<Vec<News>,Error> {
        use crate::schema::news::dsl::*;

        Ok(news
            .order_by(creation_datetime)
            .limit(8)
            .get_results::<News>(conn)?)
    }

}