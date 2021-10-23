use actix_web::{
    delete, post, get,
    web::{Json, Path},
    HttpResponse,
};
use crate::sql::store::user::{User, NewUser};
use crate::api::error::ServerError;

use serde_json::json;

#[post("/user")]
pub async fn create(user: Json<NewUser>) -> Result<HttpResponse, ServerError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/user/{chat_id}")]
pub async fn find(chat_id: Path<i64>) -> Result<HttpResponse, ServerError> {
    let user = User::find(chat_id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/user/{chat_id}")]
pub async fn delete(chat_id: Path<i64>) -> Result<HttpResponse, ServerError> {
    User::delete(chat_id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({})))
}
