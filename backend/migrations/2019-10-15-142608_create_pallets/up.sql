-- Your SQL goes here
CREATE TABLE pallets (
    id SERIAL PRIMARY KEY,
    item_code VARCHAR(100) REFERENCES items(code) ON DELETE CASCADE NOT NULL,
    warehouse_id VARCHAR(100) REFERENCES warehouses(address) ON DELETE CASCADE NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);
