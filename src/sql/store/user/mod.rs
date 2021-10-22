use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::user_models::user::{NewUser, User};
use crate::schema::users::dsl::{chat_id, user_role, users};
use crate::sql::db::DbActor;


#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Create {
    pub chat_id: i64,
    pub user_role: i32,
    pub create_at: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct Get{
    pub chat_id: i64,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Update {
    pub chat_id: i64,
    pub user_role: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct Delete {
    pub chat_id: i64,
}

/**
 * Реализация инструкции добавления
 * новго пользователя в БД.
 */
impl Handler<Create> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        // Установка соединения
        let conn = self.0.get().expect("Unable to get a connectio");

        // Инициализауия нового пользователя
        let new_user = NewUser {
            chat_id: msg.chat_id,
            user_role: msg.user_role,
            create_at: msg.create_at,
        };

        // Выполняю операцию вставки новго пользователя в
        // БД в таблицу users
        diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&conn)
    }
}

/**
 * Реализация инструкции обновления
 * пользователя в БД.
 */
impl Handler<Update> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        // Выполняю операцию обновления пользователя
        diesel::update(users)
            .filter(chat_id.eq(msg.chat_id)) // предикат
            .set(user_role.eq(msg.user_role))
            .get_result::<User>(&conn)
    }
}

/**
 * Реализация инструкции удаления
 * пользователя из БД.
 */
impl Handler<Delete> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        // Выполняю операцию удаления пользователя
        diesel::delete(users)
            .filter(chat_id.eq(msg.chat_id)) // предикат
            .get_result::<User>(&conn)
    }
}

/**
 * Реализация инструкции получения
 * пользователя из БД.
 */
impl Handler<Get> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        // Выполняю операцию фильтрации для получения пользователя из БД
        users.filter(chat_id.eq(msg.chat_id))
            .get_results::<User>(&conn)
    }
}
