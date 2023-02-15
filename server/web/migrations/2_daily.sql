CREATE TABLE daily (
  id INTEGER PRIMARY KEY,
  game_id TEXT,
  FOREIGN KEY(game_id) REFERENCES games(id)
);

CREATE TABLE dates (
  date TEXT PRIMARY KEY
  daily_id INTEGER
  FOREIGN KEY(daily_id) REFERENCES daily(id)
);