extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

pub mod authentication;
pub mod database;
pub mod schema;

pub struct ErrorPossible {
    pub succeed: bool,
    pub message: String,
}