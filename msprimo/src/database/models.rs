use chrono::NaiveDate;
use schema::stories;

#[derive(Queryable)]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_date: NaiveDate,
}

#[derive(Insertable)]
#[table_name="stories"]
pub struct NewStory<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
    pub publish_date: &'a NaiveDate,
}