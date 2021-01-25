CREATE TABLE users (
    username    VARCHAR(128)    PRIMARY KEY NOT NULL,
    first_login BOOLEAN                     NOT NULL    DEFAULT TRUE
)