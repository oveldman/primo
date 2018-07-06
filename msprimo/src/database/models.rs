use chrono::NaiveDate;
use schema::stories;
use schema::accounts;
use schema::sessions;

#[derive(Queryable)]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_date: NaiveDate,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="stories"]
pub struct NewStory<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
    pub publish_date: &'a NaiveDate,
    pub user_id: &'a i32,
}

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'b> {
    pub username: &'b str,
    pub password: &'b str,
    pub first_name: &'b str,
}

#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub cookie_token: String,
}

#[derive(Insertable)]
#[table_name="sessions"]
pub struct NewSession<'c> {
    pub user_id: &'c i32,
    pub cookie_token: &'c str,
}