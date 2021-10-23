use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub db: Db,
}

#[derive(Deserialize)]
pub struct Server {
    pub port: u16,
    pub ip: String,
}

#[derive(Deserialize)]
pub struct Db {
    pub dev_url: String,
}

/**
 * Метод для парсинга конфигурации
 */
pub fn read() -> std::io::Result<Config> {
    let content = std::fs::read_to_string("config/server.toml")?;
    Ok(toml::from_str(&content)?)
}
