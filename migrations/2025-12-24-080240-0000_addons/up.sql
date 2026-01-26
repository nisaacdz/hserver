-- Your SQL goes here

CREATE TABLE addons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    -- Simple pricing model for now (per unit)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_addons_modtime
BEFORE UPDATE ON addons
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE booking_addons (
    booking_id UUID NOT NULL REFERENCES bookings(block_id) ON DELETE CASCADE,
    addon_id UUID NOT NULL REFERENCES addons(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    price_at_booking DECIMAL(10, 2) NOT NULL, -- Snapshot of price
    PRIMARY KEY (booking_id, addon_id)
);
