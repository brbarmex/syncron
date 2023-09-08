-- Create the "db_content_snapshot" database
CREATE DATABASE db_snapshot;

-- Connect to the "db_content_snapshot" database
\c db_snapshot

-- Create the "snapshots" table
CREATE TABLE IF NOT EXISTS snapshots (
    id SERIAL PRIMARY KEY,
    content_value TEXT NOT NULL,
    version TEXT NOT NULL,
    file_id TEXT NOT NULL
);

ALTER TABLE snapshots
ADD CONSTRAINT file_id_unique UNIQUE (file_id);

-- Inser a data
INSERT INTO snapshots (id, content_value, "version", file_id) VALUES(1, '', '', 'object_plan.txt');

