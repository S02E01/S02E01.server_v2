extern crate actix;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_redis::RedisSession;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
mod api;
mod schema;
mod sql;

#[cfg(test)]
mod tests;

use sql::db;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Подгружаю файл окружения
    dotenv().ok();

    // Инициализирую базу данных
    db::init();

    // Конфигурационные данные  веб-сервера
    let server_host = env::var("HOST").expect("Host not set");
    let server_port = env::var("PORT").expect("Port not set");

    // Конфигурационные данные для redis хранилища
    let redis_port = env::var("REDIS_PORT").expect("Redis port not set");
    let redis_host = env::var("REDIS_HOST").expect("Redis host not set");

    // Инициализирую HTTP сервер
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api-v2")
                .wrap(RedisSession::new(
                    format!("{}:{}", redis_host, redis_port),
                    &[0; 32],
                ))
                .configure(api::routes::public)
                .configure(api::routes::private),
        )
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
