-- Your SQL goes here

CREATE TABLE rules (
    id SERIAL PRIMARY KEY,
    warehouse VARCHAR(100) REFERENCES warehouse(id) ON DELETE CASCADE NOT NULL,
    item VARCHAR(100) NOT NULL,
    minimum INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    decription TEXT,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
)
