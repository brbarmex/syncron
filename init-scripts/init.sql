-- Create the "db_content_snapshot" database
CREATE DATABASE db_content_snapshot;

-- Connect to the "db_content_snapshot" database
\c db_content_snapshot

-- Create the "content" table
CREATE TABLE IF NOT EXISTS content (
    id SERIAL PRIMARY KEY,
    content_value TEXT NOT NULL,
    version TEXT NOT NULL
);

