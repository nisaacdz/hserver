-- Custom ENUM Types for status fields
CREATE TYPE operational_status AS ENUM ('Available', 'Cleaning', 'Maintenance');
CREATE TYPE booking_status AS ENUM ('Pending', 'Confirmed', 'Cancelled', 'Completed');
CREATE TYPE payment_status AS ENUM ('Pending', 'Success', 'Failed');

-- User & Access Control Tables
CREATE TABLE roles (
    role_id SERIAL PRIMARY KEY,
    role_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE permissions (
    permission_id SERIAL PRIMARY KEY,
    permission_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE role_permissions (
    role_id INTEGER NOT NULL REFERENCES roles(role_id) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permissions(permission_id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE staff (
    staff_id SERIAL PRIMARY KEY,
    username VARCHAR UNIQUE NOT NULL,
    hashed_password VARCHAR NOT NULL,
    role_id INTEGER NOT NULL REFERENCES roles(role_id)
);

CREATE TABLE guests (
    guest_id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR,
    hashed_password VARCHAR
);

-- Hotel & Inventory Tables
CREATE TABLE hotels (
    hotel_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address VARCHAR
);

CREATE TABLE room_types (
    room_type_id SERIAL PRIMARY KEY,
    hotel_id INTEGER NOT NULL REFERENCES hotels(hotel_id),
    name VARCHAR NOT NULL,
    description TEXT,
    base_price DECIMAL(10, 2) NOT NULL,
    max_occupancy INTEGER NOT NULL
);

CREATE TABLE rooms (
    room_id SERIAL PRIMARY KEY,
    room_type_id INTEGER NOT NULL REFERENCES room_types(room_type_id),
    hotel_id INTEGER NOT NULL REFERENCES hotels(hotel_id),
    room_number VARCHAR NOT NULL,
    operational_status operational_status NOT NULL DEFAULT 'Available'
);

-- Transactional Tables
CREATE TABLE bookings (
    booking_id SERIAL PRIMARY KEY,
    guest_id INTEGER NOT NULL REFERENCES guests(guest_id),
    room_id INTEGER NOT NULL REFERENCES rooms(room_id),
    checkin_date DATE NOT NULL,
    checkout_date DATE NOT NULL,
    total_price DECIMAL(10, 2) NOT NULL,
    booking_status booking_status NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT checkin_before_checkout CHECK (checkin_date < checkout_date)
);

CREATE TABLE payments (
    payment_id SERIAL PRIMARY KEY,
    booking_id INTEGER NOT NULL REFERENCES bookings(booking_id),
    amount DECIMAL(10, 2) NOT NULL,
    payment_status payment_status NOT NULL DEFAULT 'Pending',
    provider_txn_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE services (
    service_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    price DECIMAL(10, 2) NOT NULL
);

CREATE TABLE booking_services (
    booking_id INTEGER NOT NULL REFERENCES bookings(booking_id),
    service_id INTEGER NOT NULL REFERENCES services(service_id),
    quantity INTEGER NOT NULL,
    PRIMARY KEY (booking_id, service_id)
);
