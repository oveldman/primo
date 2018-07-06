use database::account;
use ErrorPossible;
use database::models::Account;
use rocket::http::{Cookie, Cookies};

// TODO add anti bruteforge attack
pub fn login<'b>(mut cookies: Cookies, username: &'b str, password: &'b str) -> ErrorPossible {
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

pub fn check_authorization(){

}

// TODO cookie token system
fn generate_token<'b>(mut cookies: Cookies, user_id: &'b i32) {
    cookies.add(Cookie::new("identity", "some_token"));
}