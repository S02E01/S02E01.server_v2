use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ServerError {
    pub status_code: u16,
    pub message: String,
}

impl ServerError {
    // Конструктор для создания объекта серверной ошибки
    pub fn create(status_code: u16, message: String) -> ServerError {
        ServerError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

/**
 * Список реализации методов для кастомных ошибок
*/
impl From<DieselError> for ServerError {
    fn from(error: DieselError) -> ServerError {
        match error {
            // Если возникла ошибка базы
            DieselError::DatabaseError(_, err) => {
                ServerError::create(409, err.message().to_string())
            }

            // Если нужная запись не найдена
            DieselError::NotFound => {
                ServerError::create(404, "Record not found".to_string())
            },

            // Если вообще не контролируем ситуацию :)
            err => {
                ServerError::create(500, format!("Diesel error: {}", err))
            }
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                format!("{}", self.message);
                "Internal server error".to_string()
            },
        };

        HttpResponse::build(status_code)
        .json(json!({ "message": message }))
    }
}