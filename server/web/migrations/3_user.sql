CREATE TABLE user (
  id INTEGER PRIMARY KEY,
  email TEXT UNIQUE,
  username TEXT UNIQUE,
  password_hash TEXT
)