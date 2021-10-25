use crate::api::error::ServerError;
use crate::diesel::prelude::*;
use crate::schema::users;
use crate::schema::users::dsl::{updated_at, role};
use crate::sql::db;
use argon2::Config;
use chrono::Utc;
use rand::Rng;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

/**
 * Структура объекта пользователя для создания новой записи в БД
 */
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub chat_id: i64,
    #[serde(skip_serializing)]
    pub hash: String,
    pub role: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub chat_id: i64,
    pub role: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub chat_id: i64,
}

impl User {
    /**
     * Создать запись в таблице `users`
     */
    pub fn create(user: NewUser) -> Result<Self, ServerError> {
        let conn = db::connection()?;

        // Создаю объект пользователя с автозаполнением внутренних полей
        let mut user = User::from(user);

        // Создаю хеш пользователя
        user.hashing()?;

        // Записываю пользователя в БД
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(&conn)?;

        Ok(user)
    }

    /**
     * Найти запись в таблице `users`
     */
    pub fn find(chat_id: i64) -> Result<Self, ServerError> {
        let conn = db::connection()?;

        let user = users::table
            .filter(users::chat_id.eq(chat_id))
            .first::<User>(&conn)?;

        Ok(user)
    }

    /**
     * Удалить запись в таблице `users`
     */
    pub fn delete(chat_id: i64) -> Result<usize, ServerError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            users::table
                //
                .filter(users::chat_id.eq(chat_id)),
        )
        .execute(&conn)?;

        Ok(res)
    }

    /**
     * Обновить user_role записи в таблице `users`
     */
    pub fn update(chat_id: i64) -> Result<Self, ServerError> {
        let conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::chat_id.eq(chat_id))
            .set((role.eq(2), updated_at.eq(Utc::now().naive_utc())))
            .get_result::<User>(&conn)?;

        Ok(user)
    }

    /**
     * Вспомогательный метод для создания хеша пользователя
     */
    fn hashing(&mut self) -> Result<(), ServerError> {
        // Создаю случайную 32-битную соль
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.hash = argon2::hash_encoded(self.hash.as_bytes(), &salt, &config)
            .map_err(|error| ServerError::create(500, format!("Failed create hash: {}", error)))?;

        Ok(())
    }

    /**
     * Вспомогательный метод верификации хеша и входящего chat_id
     */
    pub fn verify(&self, chat_id: &[u8]) -> Result<bool, ServerError> {
        argon2::verify_encoded(&self.hash, chat_id)
        .map_err(|error| ServerError::create(500, format!("Failed to verify password: {}", error)))
    }
}

impl From<NewUser> for User {
    /**
     * Метод автозаполнение внутренних полей 
     */
    fn from(new_user: NewUser) -> Self {
        User {
            chat_id: new_user.chat_id,
            hash: format!("{}", new_user.chat_id),
            role: new_user.role,            
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
