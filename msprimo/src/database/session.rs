use database::establish_connection;
use diesel::insert_into;
use diesel::prelude::*;
use database::models::Session;
use database::models::NewSession;
use diesel::delete;
use schema::sessions;
use schema::sessions::dsl::*;

pub fn new_session<'c>(token: &'c str, logged_user_id: &'c i32) -> bool {
    remove_session(logged_user_id);

    let connection = establish_connection();

    let new_session = NewSession {
        user_id: logged_user_id,
        cookie_token: token,
    };

    insert_into(sessions::table)
        .values(&new_session)
        .execute(&connection)
        .expect("Error saving new session");

    true
}

pub fn get_session<'c>(token: &'c str) -> Vec<Session> {
    let connection = establish_connection();
    let search_session = sessions.filter(cookie_token.eq(token))
        .limit(1)
        .load::<Session>(&connection)
        .expect("Error loading session");

    let mut found_session: Vec<Session> = Vec::new();

    for session in search_session {
        found_session.push(session);
    }

    found_session
}

pub fn remove_session<'c>(logged_user_id: &'c i32) -> bool{
    let connection = establish_connection();

    delete(sessions.filter(user_id.eq(logged_user_id)))
        .execute(&connection)
        .expect("Error deleting session");

    true
}