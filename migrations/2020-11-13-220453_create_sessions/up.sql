CREATE TABLE sessions (
  auth_token VARCHAR(128) PRIMARY KEY NOT NULL,
  expiration_time BIGINT NOT NULL,
  username TEXT NOT NULL,
  smartape_token TEXT NOT NULL
)