CREATE TABLE system_messages (
    id             INTEGER PRIMARY KEY AUTO_INCREMENT NOT NULL,
    user           TEXT    NOT NULL,
    messageType    TEXT    NOT NULL,
    time           BIGINT  NOT NULL,
    content        TEXT    NOT NULL
)