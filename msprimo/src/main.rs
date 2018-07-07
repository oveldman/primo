//cargo install diesel_cli

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;
extern crate msprimo;

use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;
use std::path::Path;
use msprimo::{authentication, ErrorPossible, ViewBag, TOKEN_NAME};
use rocket::http::Cookies;
use rocket::request::Form;

#[derive(FromForm)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

//TODO Restyle the login page

#[get("/")]
fn index(cookies: Cookies) -> Template{
    let admin_right: bool = check_authentication(&cookies);
    let view_bag: ViewBag = get_view_bag(false, admin_right, String::from(""));
    Template::render("index", view_bag)
}

#[get("/test")]
fn test_method(cookies: Cookies) -> String {
    let check: bool = check_authentication(&cookies);
    //let done: ErrorPossible = authentication::sign_in("test2@test.nl","Test1234", "Oscar");
    //done.message.to_string()

    String::from("Is there a session from the cookie: ") + &check.to_string()
}

#[get("/login")]
fn login(cookies: Cookies) -> Template {
    let view_bag: ViewBag = get_view_bag(true, false, String::from(""));
    Template::render("login", view_bag)
}

#[post("/login", data = "<credentials>")]
fn set_login(cookies: Cookies, credentials: Form<Credentials>) -> Result<Redirect, Template>{
    let done: ErrorPossible = authentication::login(cookies, &credentials.get().username, &credentials.get().password);

    if done.succeed {
        Ok(Redirect::to("/"))
    } else{
        let view_bag: ViewBag = get_view_bag(true, false, done.message);
        Err(Template::render("login", view_bag))
    }

}

#[get("/logout")]
fn logout(cookies: Cookies) -> Redirect {
    let token: String = get_login_token(&cookies);
    authentication::logout(cookies, &token);
    Redirect::to("/")
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
        .mount("/", routes![index, test_method, login, set_login, logout, content_files, bootstrap_files])
        .attach(Template::fairing())
        .catch(errors![not_found])
        .launch();
}

fn check_authentication<'c>(cookies: &'c Cookies) -> bool {
    let current_token: String = get_login_token(cookies);
    authentication::check_authorization(&current_token)
}

fn get_login_token<'c>(cookies: &'c Cookies) -> String {
    let mut current_token_cookie: String = String::from("");
    cookies.get(TOKEN_NAME).map(|value| current_token_cookie = value.to_string());
    let splitted_field_cookie: Vec<&str> = current_token_cookie.split('=').collect();
    let mut current_token: &str = "";

    if splitted_field_cookie.len() > 1 {
        current_token = &splitted_field_cookie[1];
    }

    current_token.to_string()
}

fn get_view_bag(sign_in: bool, admin_access: bool, login_error: String) -> ViewBag {

    let mut login_wrong: bool = false;

    if login_error.len() > 0 {
        login_wrong = true;
    }

    let new_view_bag = ViewBag {
        sign_in,
        login_wrong,
        login_error,
        admin_access,
    };

    new_view_bag
}