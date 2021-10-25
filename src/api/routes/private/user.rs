use crate::api::error::ServerError;
use crate::sql::store::user_repository::user::{User, UpdateUser};
use actix_session::Session;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use actix_web::{delete, get, put};
use serde_json::json;

/**
 * Метод для получение объекта
 * пользователя из таблицы `users`.
 */
#[get("/user/{chat_id}")]
pub async fn find(chat_id: Path<i64>, session: Session) -> Result<HttpResponse, ServerError> {
    let id: Option<i64> = session.get("id")?;

    if let Some(id) = id {
        if id == *chat_id {
            let user = User::find(chat_id.into_inner())?;
            Ok(HttpResponse::Ok().json(user))
        } else {
            Err(ServerError::create(401, "Credentials not valid!".to_string()))
        }
    } else {
        Err(ServerError::create(422, "Unauthorized".to_string()))
    }
}

/**
 * Метод для удаления записи о пользователе
 * в БД в таблице `users`.
 */
#[delete("/user/{chat_id}")]
pub async fn delete(chat_id: Path<i64>, session: Session) -> Result<HttpResponse, ServerError> {
    let id: Option<i64> = session.get("id")?;

    if let Some(id) = id {
        if id == *chat_id {
            // Проверяю, является ли пользователь админом            
            let user = User::find(chat_id.into_inner())?;
            if user.role == 2 {
                User::delete(user.chat_id)?;
                Ok(HttpResponse::Ok().json(json!({})))
            } else {
                Err(ServerError::create(404, "Page not found".to_string()))
            }
        } else {
            Err(ServerError::create(401, "Credentials not valid!".to_string()))
        }
    } else {
        Err(ServerError::create(422, "Unauthorized".to_string()))
    }
}

/**
 * Метод для обновления user_role для конкретного
 * пользователя в таблице `users`.
 *
 * Так как кроме user_role ничего не должно обновлять,
 * не использую параметры обновления.
 */
#[put("/user/{chat_id}")]
pub async fn update(chat_id: Path<i64>, data: Json<UpdateUser>, session: Session) -> Result<HttpResponse, ServerError> {
    let id: Option<i64> = session.get("id")?;    

    if let Some(id) = id {       
        if id == *chat_id {
            // Проверяю, является ли пользователь админом            
            let user = User::find(chat_id.into_inner())?;
            if user.role == 2 {
                let update_user = User::update(data.chat_id)?;
                Ok(HttpResponse::Ok().json(update_user))
            } else {
                Err(ServerError::create(404, "Page not found".to_string()))
            }
        } else {
            Err(ServerError::create(401, "Credentials not valid!".to_string()))
        }
    } else {
        Err(ServerError::create(422, "Unauthorized".to_string()))
    }
}
