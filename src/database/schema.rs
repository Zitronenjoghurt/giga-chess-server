// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        token_hash -> Varchar,
    }
}
