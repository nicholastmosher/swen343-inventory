-- Your SQL goes here
CREATE TABLE boxes (
    id SERIAL PRIMARY KEY,
    pallet_id INTEGER REFERENCES pallets(id),
    item_code VARCHAR(100) REFERENCES items(code),
    quantity INTEGER NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
)
