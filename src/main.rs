extern crate actix;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod models;
mod schema;
mod sql;

use actix_web::{
    delete, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};

use actix::SyncArbiter;
use models::{AppState, user_models::user::UserRequestData};
use sql::utils;
use sql::db::{Create, DbActor, Delete};


#[post("/user")]
async fn create_user(user: Json<UserRequestData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    match db.send(Create {chat_id: user.chat_id, user_role: user.user_role, create_at: String::from("test")}).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),    
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[delete("/{chat_id}/delete")]
async fn delete_user(Path(chat_id): Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(Delete {chat_id}).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(err)) =>{
            println!("{}", err);
            HttpResponse::NotFound().json("User not found")
        },
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = String::from("postgres://architect:password@localhost:5432/architect_dev?sslmode=disable");
    utils::run_migrations(&db_url);
    
    let pool =  utils::get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    
    HttpServer::new(move || {
        App::new()
        .service(create_user)
        .service(delete_user)
        .data(AppState {
            db: db_addr.clone()
        })
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}