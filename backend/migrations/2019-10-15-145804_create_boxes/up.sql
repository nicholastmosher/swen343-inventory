-- Your SQL goes here
CREATE TABLE boxes (
    id SERIAL PRIMARY KEY,
    pallet_id INTEGER REFERENCES pallets(id) NOT NULL,
    item_quantity INTEGER NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
)
