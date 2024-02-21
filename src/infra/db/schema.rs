// @generated automatically by Diesel CLI.

diesel::table! {
    oauth2_records (id) {
        id -> Uuid,
        #[max_length = 255]
        csrf_state -> Varchar,
        #[max_length = 255]
        pkce_code_verifier -> Varchar,
        #[max_length = 255]
        return_url -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        session_token_p1 -> Text,
        session_token_p2 -> Text,
        created_at -> Int8,
        expires_at -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    oauth2_records,
    posts,
    user_sessions,
    users,
);
