CREATE TABLE users
(
    username    VARCHAR(128) NOT NULL                   PRIMARY KEY,
    displayName TEXT         NOT NULL,
    password    TEXT                    DEFAULT NULL,
    ltiEnabled  BOOLEAN      NOT NULL   DEFAULT false,
    charData    TEXT         NOT NULL
)