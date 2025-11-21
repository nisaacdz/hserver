-- 1. Enable btree_gist to allow mixing Scalar (UUID) and Range types in one index
CREATE EXTENSION IF NOT EXISTS btree_gist;

-- 2. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'customer', -- 'customer', 'admin', 'staff'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Rooms Table
CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    number TEXT NOT NULL,
    room_type TEXT NOT NULL, -- e.g. 'Deluxe', 'Standard'
    price_per_night DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Bookings Table with CONSTRAINT
CREATE TABLE bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id),
    guest_id UUID NOT NULL REFERENCES users(id),
    stay_period TSRANGE NOT NULL, -- Time Range [start, end)
    status TEXT NOT NULL DEFAULT 'CONFIRMED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- THE GUARD RAIL:
    -- Prevents overlap only if room_id is the same AND status is not CANCELLED
    CONSTRAINT no_double_bookings EXCLUDE USING GIST (
        room_id WITH =,
        stay_period WITH &&
    ) WHERE (status != 'CANCELLED')
);
