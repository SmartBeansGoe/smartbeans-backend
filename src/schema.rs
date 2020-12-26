table! {
    achievements (id) {
        id -> Integer,
        username -> Text,
        achievementId -> BigInt,
        completionTime -> BigInt,
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
    sessions,
);
