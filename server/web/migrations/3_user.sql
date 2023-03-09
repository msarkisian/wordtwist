CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  email TEXT UNIQUE,
  username TEXT UNIQUE,
  password_hash TEXT
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_username ON users (username);