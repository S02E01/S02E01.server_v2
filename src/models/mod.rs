pub mod user_models;

use crate::sql::db::DbActor;
use crate::actix::Addr;

pub struct AppState {
   pub db: Addr<DbActor>
}