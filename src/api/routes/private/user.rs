use crate::api::error::ServerError;
use crate::sql::store::user_repository::user::{User};
use actix_web::web::{Path};
use actix_web::HttpResponse;
use actix_web::{delete, get, put};
use serde_json::json;

/**
 * Метод для получение объекта
 *  пользователя из таблицы `users`.
 */
#[get("/user/{chat_id}")]
pub async fn find(chat_id: Path<i64>) -> Result<HttpResponse, ServerError> {
    let user = User::find(chat_id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

/**
 * Метод для удаления записи о пользователе 
 * в БД в таблице `users`.
 */
#[delete("/user/{chat_id}")]
pub async fn delete(chat_id: Path<i64>) -> Result<HttpResponse, ServerError> {
    User::delete(chat_id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({})))
}

/**
 * Метод для обновления user_role для конкретного 
 * пользователя в таблице `users`.
 * 
 * Так как кроме user_role ничего не должно обновлять, 
 * не использую параметры обновления.
 */
#[put("/user/{chat_id}")]
pub async fn update(chat_id: Path<i64>) -> Result<HttpResponse, ServerError> {
    let user = User::update(chat_id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
