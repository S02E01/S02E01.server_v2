use actix_web::{web, Scope};

use crate::api::user;

pub fn router() -> Scope {
    web::scope("/api-v2")
        .service(user::get_user)
        .service(user::create_user)
        .service(user::delete_user)
}
