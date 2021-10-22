extern crate actix;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod models;
mod schema;
mod server;
mod sql;



fn main() {
    match server::start() {
        Ok(_) => println!("Good"),
        Err(_) => println!("Error"),
    }
}
