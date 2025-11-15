-- Drop tables in reverse order to respect foreign key constraints
DROP TABLE IF EXISTS booking_services;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS bookings;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS room_types;
DROP TABLE IF EXISTS hotels;
DROP TABLE IF EXISTS guests;
DROP TABLE IF EXISTS staff;
DROP TABLE IF EXISTS role_permissions;
DROP TABLE IF EXISTS permissions;
DROP TABLE IF EXISTS roles;

-- Drop custom ENUM types
DROP TYPE IF EXISTS payment_status;
DROP TYPE IF EXISTS booking_status;
DROP TYPE IF EXISTS operational_status;
