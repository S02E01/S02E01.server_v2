use serde_json::json;
use actix_web::post;

use actix_web::HttpResponse;
use actix_session::Session;
use actix_web::web::{Json, Path};

use crate::api::error::ServerError;
use crate::sql::store::user_repository::user::{NewUser, User};


/**
 * Метод для регистрации пользователя
 */
#[post("/registration")]
pub async fn registration(user: Json<NewUser>) -> Result<HttpResponse, ServerError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

/**
 * Метод для авторизации пользователя
 */
#[post("/auth/{chat_id}")]
pub async fn auth(chat_id: Path<i64>, session: Session) -> Result<HttpResponse, ServerError> {
    // Пытаюсь найти пользователя в БД
    let user = User::find(*chat_id)
        .map_err(|e| {
            match e.status_code {
                // Если такого пользователя нет
                404 => ServerError::create(422, "Credentials not valid!".to_string()),
                _ => e,
            }
        })?;
  
    // Если пользователь найден проверяю их хеши
    let is_valid = user.verify(format!("{}", *chat_id).as_bytes())?;

    if is_valid == true {
        // chat_id
        session.set("id", user.chat_id)?;
        session.renew();

        Ok(HttpResponse::Ok().json(user))
    }
    else {
        Err(ServerError::create(422, "Credentials not valid!".to_string()))
    }
}

/**
 * Метод для удаления текущей сессии пользователя
 */
#[post("/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse, ServerError> {
    let chat_id: Option<i64> = session.get("id")?;
    if let Some(_) = chat_id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({ "message": "Successfully signed out" })))
    } else {
        Err(ServerError::create(401, "Unauthorized".to_string()))
    }
}