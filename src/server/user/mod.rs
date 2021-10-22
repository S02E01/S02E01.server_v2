use actix_web::{
    delete, post, get,
    web::{Data, Json, Path},
     HttpResponse, Responder,
};
use crate::models::{AppState, user_models::user::UserRequestData};
use crate::sql::store::user::{Create, Delete, Get};

pub mod user_tests;

/**
 * При валидных данных создает нового пользователя 
 * в БД в таблице `users`
 */
#[post("/user")]
pub async fn create_user(user: Json<UserRequestData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    match db.send(Create {
        chat_id: user.chat_id, 
        user_role: user.user_role, 
        create_at: String::from("test"),
    }).await {
        Ok(Ok(user)) => HttpResponse::Created().json(user),    
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

/**
 * При валидных данных возвращает пользователя 
 * из БД таблицы `users`
 */
#[get("/user/{chat_id}")]
pub async fn get_user(Path(chat_id): Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    
    match db.send(Get {chat_id}).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => {        
            HttpResponse::NotFound().json({})
        }, 
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

/**
 * При валидных данных удаляет пользователя 
 * из БД в таблице `users`
 */
#[delete("user/{chat_id}/delete")]
pub async fn delete_user(Path(chat_id): Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(Delete {chat_id}).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) =>{         
            HttpResponse::NotFound().json({})
        },
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}
