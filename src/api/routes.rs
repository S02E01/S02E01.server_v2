use actix_web::{web, Scope};

use crate::api::user;

pub fn router() -> Scope {
    web::scope("/api-v2")
        .service(user::create)
        .service(user::find)
        .service(user::delete)
        .service(user::update)

}
