//cargo install diesel_cli

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;
extern crate msprimo;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::Path;
use msprimo::authentication;
use msprimo::ErrorPossible;
use msprimo::TOKEN_NAME;
use rocket::http::{Cookie, Cookies};

//TODO Create normal login page

#[get("/")]
fn index() -> Template{
    Template::render("index", "")
}

#[get("/test")]
fn test_method(mut cookies: Cookies) -> String {
    let current_token: String = get_login_token(cookies);
    let check: bool = authentication::check_authorization(&current_token);

    //let done: ErrorPossible = authentication::sign_in("test2@test.nl","Test1234", "Oscar");
    //done.message.to_string()
    WW
    String::from("Is there a session from the cookie: ") + &check.to_string()
}

#[get("/login")]
fn login(mut cookies: Cookies) -> String {
    let done: ErrorPossible = authentication::login(cookies,"test2@test.nl","Test1234");
    done.message.to_string()
}

#[get("/bootstrap/<file_type>/<file>")]
fn bootstrap_files(file_type: String, file: String) -> Option<NamedFile> {
    let standard_location: String = "templates/content/".to_string() + &file_type.to_string() + "/bootstrap";
    println!("{}", standard_location);
    NamedFile::open(Path::new(&standard_location).join(file)).ok()
}

#[get("/content/<file_type>/<file>")]
fn content_files(file_type: String, file: String) -> Option<NamedFile> {
    let standard_location: String = "templates/content/".to_string() + &file_type.to_string();
    NamedFile::open(Path::new(&standard_location).join(file)).ok()
}

#[error(404)]
fn not_found() -> Template {
    Template::render("404", "")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, test_method, login, content_files, bootstrap_files])
        .attach(Template::fairing())
        .catch(errors![not_found])
        .launch();
}

fn get_login_token(mut cookies: Cookies) -> String {
    let mut current_token_cookie: String = String::from("");
    cookies.get(TOKEN_NAME).map(|value| current_token_cookie = value.to_string());
    let splitted_field_cookie: Vec<&str> = current_token_cookie.split('=').collect();
    let mut current_token: &str = "";

    if splitted_field_cookie.len() > 1 {
        current_token = &splitted_field_cookie[1];
    }

    current_token.to_string()
}