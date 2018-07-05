pub mod models;

use database::models::Story;
use database::models::NewStory;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDate;
use chrono::Local;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var( "DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_stories(){
    use schema::stories::dsl::*;

    let connection = establish_connection();
    let results = stories.filter(published.eq(true))
        .limit(5)
        .load::<Story>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for story in results {
        println!("{}", story.title);
        println!("----------\n");
        println!("{}", story.body);
    }
}

pub fn upload_story<'a>(title: &'a str, body: &'a str) -> bool{
    use schema::stories;

    let connection = establish_connection();

    let current_date: NaiveDate = Local::today().naive_local();

    let new_story = NewStory {
        title,
        body,
        published: &true,
        publish_date: &current_date,
    };

    insert_into(stories::table)
        .values(&new_story)
        .execute(&connection)
        .expect("Error saving new post");

    true
}