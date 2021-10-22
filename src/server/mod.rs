use actix_web::{web, App, HttpServer};
use crate::actix::SyncArbiter;
use crate::models::AppState;
use crate::sql::db::DbActor;
use crate::sql::utils;

pub mod user;


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let db_url = String::from("postgres://architect:password@localhost:5432/architect_dev?sslmode=disable");
    utils::run_migrations(&db_url);
    
    let pool =  utils::get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    
    HttpServer::new(move || {
        App::new()        
        .service(
            web::scope("/api-v2")     
            .service(user::get_user)    
            .service(user::create_user)
            .service(user::delete_user)
        )
        .data(AppState {
            db: db_addr.clone()
        })
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
