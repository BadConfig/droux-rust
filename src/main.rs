use droux;
extern crate diesel_migrations;
use droux::db::establish_connection;
use diesel_migrations::run_pending_migrations;

fn main() {

    match run_pending_migrations(&establish_connection()) {
        Ok(_) => print!("migration success\n"),
        Err(_)=> print!("migration error\n"),
    };
    
    droux::app().launch();
}