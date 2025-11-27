-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS btree_gist;

CREATE TABLE blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id),
    interval TSTZRANGE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT no_overlaps EXCLUDE USING GIST (
        room_id WITH =,
        interval WITH &&
    )
);
