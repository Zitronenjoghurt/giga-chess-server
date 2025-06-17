// @generated automatically by Diesel CLI.

diesel::table! {
    invite_codes (id) {
        id -> Uuid,
        used -> Bool,
        comment -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        invite_code_id -> Uuid,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(users -> invite_codes (invite_code_id));

diesel::allow_tables_to_appear_in_same_query!(
    invite_codes,
    users,
);
