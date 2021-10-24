extern crate actix;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
mod api;
mod schema;
mod sql;

use sql::db;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    HttpServer::new(move || {
        App::new().service(api::routes::router())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await

}
