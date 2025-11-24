-- Your SQL goes here

CREATE TABLE housekeeping (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    instructions TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING'
);
