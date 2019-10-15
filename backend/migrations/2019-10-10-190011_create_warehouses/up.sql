-- Your SQL goes here
CREATE TABLE warehouses (
    name VARCHAR(100) PRIMARY KEY,
    address TEXT UNIQUE NOT NULL,
    description TEXT
);
