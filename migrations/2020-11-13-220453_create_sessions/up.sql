CREATE TABLE sessions (
  auth_token VARCHAR PRIMARY KEY NOT NULL,
  expiration_time BIGINT NOT NULL,
  username TEXT NOT NULL
)