-- Your SQL goes here

CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    label VARCHAR(55) NOT NULL,
    class_id UUID NOT NULL REFERENCES room_classes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
