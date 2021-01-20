CREATE TABLE characters (
    username VARCHAR(128) PRIMARY KEY NOT NULL,
    body_color TEXT DEFAULT NULL,
    hat_id TEXT DEFAULT NULL,
    face_id TEXT DEFAULT NULL,
    shirt_id TEXT DEFAULT NULL,
    pants_id TEXT DEFAULT NULL
)