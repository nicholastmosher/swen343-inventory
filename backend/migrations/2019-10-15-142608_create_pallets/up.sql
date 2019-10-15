-- Your SQL goes here
CREATE TABLE pallets (
    id SERIAL PRIMARY KEY,
    item_code VARCHAR(100) REFERENCES items(code) NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);
