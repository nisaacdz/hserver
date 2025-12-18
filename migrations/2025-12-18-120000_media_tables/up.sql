-- Your SQL goes here

CREATE TYPE media_kind AS ENUM ('image', 'video');

CREATE TABLE rooms_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id),
    external_id TEXT NOT NULL,
    caption TEXT,
    kind media_kind NOT NULL DEFAULT 'image',
    width INTEGER,
    height INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE room_classes_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    class_id UUID NOT NULL REFERENCES room_classes(id),
    external_id TEXT NOT NULL,
    caption TEXT,
    kind media_kind NOT NULL DEFAULT 'image',
    width INTEGER,
    height INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
