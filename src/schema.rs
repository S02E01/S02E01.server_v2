table! {
    users (chat_id) {
        chat_id -> Int8,
        hash -> Text,
        role -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
