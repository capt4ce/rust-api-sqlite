-- Your SQL goes here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    created_at TEXT NOT NULL
);

INSERT INTO
    "users"(name, address, created_at)
VALUES
("John", "123 Av Q", "Today");