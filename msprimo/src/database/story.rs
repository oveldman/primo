use database::establish_connection;
use database::models::Story;
use database::models::NewStory;
use diesel::insert_into;
use diesel::prelude::*;
use chrono::NaiveDate;
use chrono::Local;

pub fn get_stories(){
    use schema::stories::dsl::*;

    let connection = establish_connection();
    let results = stories.filter(published.eq(true))
        .limit(5)
        .load::<Story>(&connection)
        .expect("Error loading posts");

    let mut showed_stories: Vec<Story> = Vec::new();

    for story in results {
        showed_stories.push(story);
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
        user_id: &0,
    };

    insert_into(stories::table)
        .values(&new_story)
        .execute(&connection)
        .expect("Error saving new post");

    true
}