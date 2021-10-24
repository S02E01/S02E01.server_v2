use crate::api::error::ServerError;
use crate::diesel::prelude::*;
use crate::schema::users;
use crate::schema::users::dsl::{updated_at, user_role};
use crate::sql::db;
use chrono::Utc;

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
    pub user_role: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub chat_id: i64,
    pub user_role: i32,
}

impl User {
    pub fn create(user: NewUser) -> Result<Self, ServerError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(&conn)?;

        Ok(user)
    }

    //
    pub fn find(chat_id: i64) -> Result<Self, ServerError> {
        let conn = db::connection()?;

        let user = users::table
            .filter(users::chat_id.eq(chat_id))
            .first(&conn)?;

        Ok(user)
    }

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

    pub fn update(chat_id: i64) -> Result<Self, ServerError> {
        let conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::chat_id.eq(chat_id))
            .set((
                user_role.eq(2),
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<User>(&conn)?;

        Ok(user)
    }
}

impl From<NewUser> for User {
    fn from(new_user: NewUser) -> Self {
        User {
            chat_id: new_user.chat_id,
            user_role: new_user.user_role,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
