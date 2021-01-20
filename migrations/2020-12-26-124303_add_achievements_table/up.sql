CREATE TABLE achievements (
    id             INTEGER PRIMARY KEY AUTO_INCREMENT NOT NULL,
    username       TEXT    NOT NULL,
    achievementId  BIGINT  NOT NULL,
    completionTime BIGINT  NOT NULL
)