table! {
    users (username) {
        username -> Varchar,
        displayName -> Text,
        password -> Nullable<Text>,
        ltiEnabled -> Bool,
        charData -> Text,
    }
}
