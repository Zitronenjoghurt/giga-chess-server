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
    rooms (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        public -> Bool,
        player_white -> Nullable<Uuid>,
        player_black -> Nullable<Uuid>,
        time_micros -> Nullable<Int8>,
        increment_micros -> Nullable<Int8>,
        created_by -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        room_id -> Uuid,
        game -> Bytea,
        white_timer_micros -> Nullable<Int8>,
        black_timer_micros -> Nullable<Int8>,
        increment_micros -> Nullable<Int8>,
        last_move -> Timestamptz,
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

diesel::joinable!(sessions -> rooms (room_id));
diesel::joinable!(users -> invite_codes (invite_code_id));

diesel::allow_tables_to_appear_in_same_query!(
    invite_codes,
    rooms,
    sessions,
    users,
);
