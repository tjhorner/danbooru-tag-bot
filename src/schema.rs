table! {
    post_index (id) {
        id -> Int4,
        last_seen_post -> Int4,
    }
}

table! {
    subscriptions (id) {
        id -> Int4,
        tag -> Text,
        user_id -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    post_index,
    subscriptions,
);
