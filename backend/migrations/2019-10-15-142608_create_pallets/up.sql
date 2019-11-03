-- Your SQL goes here
CREATE TABLE pallets (
    id SERIAL PRIMARY KEY,
    item_code VARCHAR(100) REFERENCES items(item_code) ON DELETE CASCADE NOT NULL,
    warehouse_name VARCHAR(100) REFERENCES warehouses(name) ON DELETE CASCADE NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);
