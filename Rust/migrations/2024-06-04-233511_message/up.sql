-- Your SQL goes here
CREATE TABLE test (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL
);

INSERT INTO test (content) VALUES ('Hello, world!');