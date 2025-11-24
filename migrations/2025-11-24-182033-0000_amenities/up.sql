-- Your SQL goes here

CREATE TABLE amenities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    icon_key TEXT
);
