-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);