use crate::api::error::ServerError;

pub fn check(t: i64) -> Result<bool, ServerError> {
    println!("hi it middleware!");
    Ok(true)
}