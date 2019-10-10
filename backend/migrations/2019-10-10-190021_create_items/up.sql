-- Your SQL goes here
CREATE TABLE items (
    code VARCHAR(100) PRIMARY KEY,
    cost INTEGER NOT NULL,
    description TEXT,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);
