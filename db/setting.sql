CREATE TABLE room_temp (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  temp FLOAT,
  humidity FLOAT,
  pressure FLOAT,
  updated_at DATETIME NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE TABLE app_slot (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  app_name TEXT NOT NULL,
  slot INTEGER NOT NULL
);