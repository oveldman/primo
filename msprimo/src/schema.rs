table! {
    stories (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        publish_date -> Date,
    }
}
