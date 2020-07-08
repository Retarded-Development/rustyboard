table! {
    use diesel::sql_types::*;
    use diesel::types::Int4;

    boards (board_id) {
        name -> Text,
        title -> Nullable<Text>,
        board_id -> Int4,
        last_bumped -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel::types::Int4;

    posts (post_id) {
        name -> Nullable<Text>,
        text -> Text,
        post_id -> Int4,
        board_id -> Nullable<Int4>,
        ip -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

joinable!(posts -> boards (board_id));

allow_tables_to_appear_in_same_query!(
    boards,
    posts,
);
