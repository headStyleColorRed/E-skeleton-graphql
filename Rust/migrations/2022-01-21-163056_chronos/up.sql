-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    status INT NOT NULL
);

CREATE TABLE punches (
    id SERIAL PRIMARY KEY,
    parent_id INT NOT NULL,
    entry VARCHAR NOT NULL,
    leave VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES users(id)
);