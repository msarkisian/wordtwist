CREATE TABLE scores (
  id INTEGER PRIMARY KEY,
  game_id TEXT,
  user_id INTEGER,
  score, INTEGER,
  FOREIGN KEY(game_id) REFERENCES games(id)
  FOREIGN KEY(user_id) REFERENCES users(id)
);