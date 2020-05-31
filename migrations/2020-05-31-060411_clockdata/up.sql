CREATE TABLE clockdata (
    id SERIAL PRIMARY KEY,
    datetime TIMESTAMP WITHOUT TIME ZONE,
    value TEXT,
    comment TEXT
)