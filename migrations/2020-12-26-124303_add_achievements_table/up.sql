CREATE TABLE achievements (
    id             INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username       TEXT    NOT NULL,
    achievementId  BIGINT  NOT NULL,
    completionTime BIGINT  NOT NULL
)