-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS citext;

CREATE OR REPLACE FUNCTION generate_reservation_code()
RETURNS TEXT AS $$
DECLARE
    chars TEXT := 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    result TEXT := '';
    i INTEGER;
BEGIN
    FOR i IN 1..7 LOOP
        result := result || substr(chars, floor(random() * length(chars) + 1)::integer, 1);
    END LOOP;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE TYPE reservation_status AS ENUM ('pending', 'confirmed', 'cancelled', 'expired');

CREATE TABLE reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code CITEXT NOT NULL DEFAULT generate_reservation_code(),
    guest_id UUID NOT NULL REFERENCES users(id),
    status reservation_status NOT NULL DEFAULT 'pending',
    total_amount DECIMAL(10, 2) NOT NULL,
    currency TEXT NOT NULL,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT reservations_code_key UNIQUE (code)
);

CREATE INDEX idx_reservations_expires_at ON reservations(expires_at);

CREATE TRIGGER update_reservations_modtime
BEFORE UPDATE ON reservations
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE bookings_reservations (
    booking_id UUID NOT NULL REFERENCES bookings(block_id) ON DELETE CASCADE,
    reservation_id UUID NOT NULL REFERENCES reservations(id) ON DELETE CASCADE,
    PRIMARY KEY (booking_id, reservation_id)
);

CREATE INDEX idx_reservations_guest ON reservations(guest_id);
CREATE INDEX idx_bk_res_reservation ON bookings_reservations(reservation_id);
