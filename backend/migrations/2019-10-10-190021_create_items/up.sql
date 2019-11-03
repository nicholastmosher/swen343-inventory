-- Your SQL goes here
CREATE TABLE items (
    item_code VARCHAR(100) PRIMARY KEY,
    item_type VARCHAR(100),
    cost INTEGER NOT NULL,
    description TEXT
);
