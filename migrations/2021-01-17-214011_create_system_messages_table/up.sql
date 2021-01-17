CREATE TABLE system_messages (
    id             INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user           TEXT    NOT NULL,
    messageType    TEXT    NOT NULL,
    time           BIGINT  NOT NULL,
    content        TEXT    NOT NULL
)