CREATE TABLE sessions
(
    token           VARCHAR(128)    NOT NULL                    PRIMARY KEY,
    username        VARCHAR(128)    NOT NULL,
    courseName      VARCHAR(128)    NOT NULL,
    expirationTime  BIGINT          NOT NULL,
    tokenName       TEXT                        DEFAULT NULL
)