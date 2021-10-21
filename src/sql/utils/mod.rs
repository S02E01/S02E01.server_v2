use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

/**
 * Метод для запуска всех миграций
 */
#[allow(dead_code)]
pub fn run_migrations(db_url: &str) {
    embed_migrations!();

    // Пытаюсь подключится к БД
    let connection = PgConnection::establish(db_url).expect("Error connecting to database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

/**
 * Метод для установки соединения с БД
 */
#[allow(dead_code)]
pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}