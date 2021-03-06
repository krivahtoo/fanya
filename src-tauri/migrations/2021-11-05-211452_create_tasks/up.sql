-- Your SQL goes here
CREATE TABLE tasks (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  task_type VARCHAR NOT NULL,
  tags VARCHAR,
  body TEXT NOT NULL DEFAULT '',
  done BOOLEAN NOT NULL DEFAULT 0,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  due_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
)
