-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks
(
    id     SERIAL PRIMARY KEY,
    title  VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(7)          NOT NULL
)