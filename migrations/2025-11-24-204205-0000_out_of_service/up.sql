-- Your SQL goes here

CREATE TABLE out_of_service (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    staff_id UUID NOT NULL REFERENCES staff(id)
);
