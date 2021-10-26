use actix_web::web;

pub mod private;
pub mod public;

mod tests;


/**
 * Сборка публичных маршрутов
 */
pub fn public(cfg: &mut web::ServiceConfig) {
    cfg.service(public::auth::registration);
    cfg.service(public::auth::auth);
    cfg.service(public::auth::logout);
}

/**
 * Сборка приватных маршрутов
 */
pub fn private(cfg: &mut web::ServiceConfig){
    cfg.service(private::user::find);
    cfg.service(private::user::delete);
    cfg.service(private::user::update);
}

