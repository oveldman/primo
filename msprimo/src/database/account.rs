use database::establish_connection;
use database::models::Account;
use database::models::NewAccount;
use diesel::insert_into;
use diesel::prelude::*;

pub fn get_account<'b>(chosen_username: &'b str) -> Vec<Account> {
    use schema::accounts::dsl::*;

    let connection = establish_connection();
    let search_account = accounts.filter(username.eq(chosen_username))
        .limit(1)
        .load::<Account>(&connection)
        .expect("Error loading account");

    let mut found_accounts: Vec<Account> = Vec::new();

    for account in search_account {
        found_accounts.push(account);
    }

    found_accounts
}

pub fn new_account<'b>(username: &'b str, password: &'b str, first_name: &'b str) -> bool {
    use schema::accounts;

    let connection = establish_connection();

    let new_account = NewAccount {
        username,
        password,
        first_name,
    };

    insert_into(accounts::table)
        .values(&new_account)
        .execute(&connection)
        .expect("Error saving new post");

    true
}
