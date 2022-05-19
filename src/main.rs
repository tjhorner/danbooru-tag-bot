#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

mod db;
mod bot;
mod api;
mod notifier;
mod schema;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    
    let args: Vec<String> = env::args().collect();

    let db_conn = db::establish_db_connection();
    if let Err(e) = embedded_migrations::run(&db_conn) {
        log::error!("Error running migrations: {}", e);
    }

    match args[1].as_str() {
        "bot" => {
            bot::run(db_conn).await;
        },
        "notifier" => {
            notifier::run(db_conn).await;
        },
        _ => {
            println!("Usage: cargo run bot|notifier");
        }
    }
}
