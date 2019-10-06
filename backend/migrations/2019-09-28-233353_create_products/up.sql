-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    price INTEGER NOT NULL,
    description TEXT,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);