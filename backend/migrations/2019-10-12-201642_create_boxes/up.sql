-- Your SQL goes here

CREATE TABLE boxes (
    code VARCHAR(100) PRIMARY KEY,
    quantity INT,
    pallet_id VARCHAR(100) REFERENCES pallets(id),
    deleted BOOLEAN NOT NULL DEFAULT FALSE
)
