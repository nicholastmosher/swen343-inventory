 -- Your SQL goes here

CREATE TABLE rules (
    id SERIAL PRIMARY KEY,
    warehouse_name VARCHAR(100) REFERENCES warehouses(name) ON DELETE CASCADE NOT NULL,
    item VARCHAR(100) REFERENCES items(item_code) ON DELETE CASCADE NOT NULL,
    minimum INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    description TEXT,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
)
