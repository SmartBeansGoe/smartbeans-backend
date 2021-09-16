table! {
    courseMapping (studipId) {
        studipId -> Varchar,
        courseName -> Varchar,
    }
}

table! {
    courses (name) {
        name -> Varchar,
        title -> Text,
    }
}

table! {
    sessions (token) {
        token -> Varchar,
        username -> Varchar,
        courseName -> Varchar,
        expirationTime -> Bigint,
        tokenName -> Nullable<Text>,
    }
}

table! {
    users (username) {
        username -> Varchar,
        displayName -> Text,
        password -> Nullable<Text>,
        ltiEnabled -> Bool,
        charData -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    courseMapping,
    courses,
    sessions,
    users,
);
