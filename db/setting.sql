CREATE TABLE room_temp (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  temp FLOAT,
  humidity FLOAT,
  pressure FLOAT,
  updated_at DATETIME DEFAULT (datetime('now', '+9 hours'))
);

CREATE TABLE app_slot (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  app_name TEXT NOT NULL,
  slot INTEGER NOT NULL
);

-- -- 1. 2026-05-16 01時台のデータ（秒数をズラして3つ）
-- INSERT INTO room_temp (temp, humidity, pressure, updated_at) VALUES (20.5, 50.0, 1013.2, '2026-05-16 01:00:15');
-- INSERT INTO room_temp (temp, humidity, pressure, updated_at) VALUES (20.6, 50.1, 1013.1, '2026-05-16 01:15:00');
-- INSERT INTO room_temp (temp, humidity, pressure, updated_at) VALUES (20.7, 50.2, 1013.0, '2026-05-16 01:45:30');

-- -- 2. 2026-05-16 02時台のデータ（秒数をズラして2つ）
-- INSERT INTO room_temp (temp, humidity, pressure, updated_at) VALUES (21.0, 48.5, 1012.8, '2026-05-16 02:00:05');
-- INSERT INTO room_temp (temp, humidity, pressure, updated_at) VALUES (21.2, 48.0, 1012.5, '2026-05-16 02:30:00');

-- -- 3. 2026-05-16 03時台のデータ（1つだけ）
-- INSERT INTO room_temp (temp, humidity, pressure, updated_at) VALUES (21.5, 47.0, 1012.0, '2026-05-16 03:00:10');

-- -- 4. 【本番】DEFAULT値のテスト（現在時刻がJSTで入るか確認）
-- -- updated_at を指定しないことで、設定した DEFAULT (datetime('now', '+9 hours')) が動きます
-- INSERT INTO room_temp (temp, humidity, pressure) VALUES (22.0, 45.0, 1011.5);