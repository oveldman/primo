//cargo install diesel_cli

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate msprimo;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::Path;
use msprimo::authentication;
use msprimo::ErrorPossible;
use rocket::http::{Cookie, Cookies};

#[get("/")]
fn index() -> Template{
    Template::render("index", "")
}

#[get("/test")]
fn test_method() -> String {
    let done: ErrorPossible = authentication::sign_in("test2@test.nl","Test1234", "Oscar");
    done.message.to_string()
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

fn get_login_token(){

}