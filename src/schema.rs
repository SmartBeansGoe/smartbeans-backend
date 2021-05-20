table! {
    achievements (id) {
        id -> Integer,
        username -> Text,
        achievementId -> Bigint,
        completionTime -> Bigint,
    }
}

table! {
    characters (username) {
        username -> Varchar,
        body_color -> Nullable<Text>,
        hat_id -> Nullable<Text>,
        face_id -> Nullable<Text>,
        shirt_id -> Nullable<Text>,
        pants_id -> Nullable<Text>,
    }
}

table! {
    charnames (username) {
        username -> Varchar,
        charname -> Text,
    }
}

table! {
    error_messages (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
    }
}

table! {
    error_reports (id) {
        id -> Integer,
        time -> Bigint,
        username -> Varchar,
        message -> Text,
    }
}

table! {
    route_log (id) {
        id -> Integer,
        time -> Bigint,
        username -> Varchar,
        route -> Text,
        data -> Nullable<Text>,
    }
}

table! {
    sessions (auth_token) {
        auth_token -> Varchar,
        expiration_time -> Bigint,
        username -> Text,
        smartape_token -> Text,
    }
}

table! {
    survey (id) {
        id -> Integer,
        val -> Text,
    }
}

table! {
    system_messages (id) {
        id -> Integer,
        user -> Text,
        messageType -> Text,
        time -> Bigint,
        content -> Text,
    }
}

table! {
    users (username) {
        username -> Varchar,
        first_login -> Bool,
        total_score -> Bigint,
        char_changed -> Bigint,
        studip_userid -> Text,
        survey -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    achievements,
    characters,
    charnames,
    error_messages,
    error_reports,
    route_log,
    sessions,
    survey,
    system_messages,
    users,
);
