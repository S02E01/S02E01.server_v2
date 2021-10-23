extern crate actix;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use std::io::{Error, ErrorKind};

use crate::actix::SyncArbiter;
use crate::models::AppState;
use crate::sql::db::DbActor;
use crate::sql::utils;

mod api;
mod config;
mod models;
mod schema;
mod sql;

#[actix_web::main]
pub async fn main() -> std::result::Result<(), Error> {
    // Собираю конфигурация сервераs
    match config::read() {
        Ok(conf) => {
            utils::run_migrations(&conf.db.dev_url);

            let pool = utils::get_pool(&conf.db.dev_url);
            let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
            HttpServer::new(move || {
                App::new().service(api::routes::router()).data(AppState {
                    db: db_addr.clone(),
                })
            })
            .bind((conf.server.ip, conf.server.port))?
            .run()
            .await
        }
        Err(err) => Err(Error::new(
            ErrorKind::Other,
            format!("Failed to read config file. Error: {}", err),
        ))?,
    }
}
