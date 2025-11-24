-- Your SQL goes here

CREATE TABLE bookings (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    guest_id UUID NOT NULL REFERENCES users(id),
    status TEXT NOT NULL DEFAULT 'CONFIRMED',
    payment_status TEXT NOT NULL DEFAULT 'PENDING'
);
