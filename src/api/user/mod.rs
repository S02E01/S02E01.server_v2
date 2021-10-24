use crate::api::error::ServerError;
use crate::sql::store::user::{NewUser, User};
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put};

use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

use serde_json::json;

#[post("/user")]
pub async fn create(user: Json<NewUser>) -> Result<HttpResponse, ServerError> {
    let user = User::create(user.into_inner())?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");

    let token = claims.sign_with_key(&key).unwrap();
    Ok(HttpResponse::Created().header("X-Hdr", token).json(user))
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

#[put("/user/{chat_id}")]
pub async fn update(chat_id: Path<i64>) -> Result<HttpResponse, ServerError> {
    let user = User::update(chat_id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
