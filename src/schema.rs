table! {
    sessions (auth_token) {
        auth_token -> Text,
        expiration_time -> BigInt,
        username -> Text,
    }
}
