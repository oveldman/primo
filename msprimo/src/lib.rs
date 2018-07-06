extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;
extern crate rocket;

pub mod authentication;
pub mod database;
pub mod schema;

pub const TOKEN_NAME: &str = "identity.token";

pub struct ErrorPossible {
    pub succeed: bool,
    pub message: String,
}