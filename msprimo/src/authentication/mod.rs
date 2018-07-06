use database::{account, session};
use ErrorPossible;
use database::models::Account;
use rocket::http::{Cookie, Cookies};
use uuid::Uuid;
use database::models::Session;
use TOKEN_NAME;

// TODO add anti bruteforge attack
pub fn login<'b>(cookies: Cookies, username: &'b str, password: &'b str) -> ErrorPossible {
    let mut error = ErrorPossible {
        succeed: false,
        message: String::from("Email or password is not correct"),
    };

    let accounts: Vec<Account> = account::get_account(username);

    if accounts.len() == 1 {
        if accounts[0].password == password {
            generate_token(cookies,&accounts[0].id);
            error.succeed = true;
            error.message = String::from("Logged in succesfully");
        }
    }

    error
}

pub fn sign_in<'b>(username: &'b str, password: &'b str, first_name: &'b str) -> ErrorPossible {
    let mut error = ErrorPossible {
        succeed: true,
        message: String::from("Account is created"),
    };

    let accounts: Vec<Account> = account::get_account(username);

    if accounts.len() == 0 {
           account::new_account(username, password, first_name);
    } else {
        error.succeed = false;
        error.message = String::from("Account already exists");
    }

    error
}

pub fn check_authorization<'b>(token: &'b str) -> bool {
    let mut found_session: bool = false;
    let current_session: Vec<Session> = session::get_session(token);

    if current_session.len() == 1 {
        found_session = true;
    }

    found_session
}

fn generate_token<'b>(mut cookies: Cookies, user_id: &'b i32) {
    let login_token = Uuid::new_v4();
    session::new_session(&login_token.to_string(), user_id);
    cookies.add(Cookie::new(TOKEN_NAME, login_token.to_string()));
}