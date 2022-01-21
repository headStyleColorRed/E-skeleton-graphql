table! {
    punches (id) {
        id -> Int4,
        parent_id -> Int4,
        entry -> Varchar,
        leave -> Varchar,
        created_at -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        status -> Int4,
    }
}

joinable!(punches -> users (parent_id));

allow_tables_to_appear_in_same_query!(
    punches,
    users,
);
