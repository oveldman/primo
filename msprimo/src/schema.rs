table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
    }
}

table! {
    stories (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        publish_date -> Date,
        user_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    stories,
);
