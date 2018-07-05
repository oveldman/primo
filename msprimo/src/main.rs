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
use rocket::Request;
use msprimo::database;

#[get("/")]
fn index() -> Template{
    Template::render("index", "")
}

#[get("/test")]
fn test_methode() -> String {
    database::upload_story("test2", "Dit is een andere test!");
    database::get_stories();
    "Done".to_string()
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
        .mount("/", routes![index, test_methode, content_files, bootstrap_files])
        .attach(Template::fairing())
        .catch(errors![not_found])
        .launch();
}