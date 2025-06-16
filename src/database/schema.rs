// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        token_hash -> Varchar,
    }
}
