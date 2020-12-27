table! {
    achievements (id) {
        id -> Integer,
        username -> Text,
        achievementId -> BigInt,
        completionTime -> BigInt,
    }
}

table! {
    characters (username) {
        username -> Text,
        body_color -> Nullable<Text>,
        hat_id -> Nullable<Text>,
        face_id -> Nullable<Text>,
        shirt_id -> Nullable<Text>,
        pants_id -> Nullable<Text>,
    }
}

table! {
    sessions (auth_token) {
        auth_token -> Text,
        expiration_time -> BigInt,
        username -> Text,
        smartape_token -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    achievements,
    characters,
    sessions,
);
