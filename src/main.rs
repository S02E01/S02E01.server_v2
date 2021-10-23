extern crate actix;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
mod api;
mod schema;
mod sql;

use sql::db;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(api::routes::router()));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    println!("Starting server");
    server.run().await

    // Собираю конфигурация сервераs
    // match config::read() {
    //     Ok(conf) => {
    //         utils::run_migrations(&conf.db.dev_url);

    //         let pool = utils::get_pool(&conf.db.dev_url);
    //         let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    //         HttpServer::new(move || {
    //             App::new().service(api::routes::router()).data(AppState {
    //                 db: db_addr.clone(),
    //             })
    //         })
    //         .bind((conf.server.ip, conf.server.port))?
    //         .run()
    //         .await
    //     }
    //     Err(err) => Err(Error::new(
    //         ErrorKind::Other,
    //         format!("Failed to read config file. Error: {}", err),
    //     ))?,
    // }
}
