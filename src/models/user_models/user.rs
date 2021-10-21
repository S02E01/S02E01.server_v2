use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::users;

/**
 * Структура объекта пользователя ПОЛУЧАЕМАЯ из БД
 */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub chat_id: i64,
    pub user_role: i32,
    pub create_at: String,
}

/**
 * Структура объекта пользователя для создания новой записи в БД
 */
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub chat_id: i64,
    pub user_role: i32,
    pub create_at: String,
}

/* Ожидаемые поля */

#[derive(Serialize, Deserialize)]
pub struct UserRequestData {
    pub chat_id: i64,
    pub user_role: i32,
}