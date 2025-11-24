-- Your SQL goes here

CREATE TABLE maintenance (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    severity TEXT NOT NULL
);
