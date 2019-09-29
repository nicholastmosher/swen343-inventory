-- Your SQL goes here
CREATE TABLE products (
    product_id SERIAL PRIMARY KEY,
    product_name TEXT NOT NULL,
    product_description TEXT
);