-- Your SQL goes here

CREATE TYPE booking_status AS ENUM ('pending', 'confirmed');

CREATE TABLE bookings (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    guest_id UUID NOT NULL REFERENCES users(id),
    status booking_status NOT NULL DEFAULT 'confirmed',
    expires_at TIMESTAMPTZ
);

CREATE INDEX idx_bookings_expires_at ON bookings(expires_at);
