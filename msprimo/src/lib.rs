extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

pub mod authentication;
pub mod database;
pub mod schema;

use rocket::request::FromForm;

pub const TOKEN_NAME: &str = "identity.token";

pub struct ErrorPossible {
    pub succeed: bool,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ViewBag {
    pub sign_in: bool,
    pub login_wrong: bool,
    pub login_error: String,
    pub admin_access: bool,
}