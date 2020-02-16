table! {
    authors (id) {
        id -> Integer,
        first_name -> Text,
    }
}

table! {
    category (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        author_id -> Integer,
        category_id -> Integer,
    }
}

joinable!(posts -> authors (author_id));
joinable!(posts -> category (category_id));

allow_tables_to_appear_in_same_query!(
    authors,
    category,
    posts,
);
