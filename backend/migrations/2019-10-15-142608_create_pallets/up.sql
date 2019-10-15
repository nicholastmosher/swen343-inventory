-- Your SQL goes here
CREATE TABLE pallets (
    id SERIAL PRIMARY KEY,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);
