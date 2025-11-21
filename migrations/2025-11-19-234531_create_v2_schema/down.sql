-- Drop Triggers
DROP TRIGGER IF EXISTS update_payments_modtime ON payments;
DROP TRIGGER IF EXISTS update_bookings_modtime ON bookings;
DROP TRIGGER IF EXISTS update_daily_rates_modtime ON daily_rates;
DROP TRIGGER IF EXISTS update_rate_plans_modtime ON rate_plans;
DROP TRIGGER IF EXISTS update_users_modtime ON users;
DROP TRIGGER IF EXISTS update_roles_modtime ON roles;
DROP TRIGGER IF EXISTS update_rooms_modtime ON rooms;
DROP TRIGGER IF EXISTS update_room_types_modtime ON room_types;
DROP TRIGGER IF EXISTS update_hotels_modtime ON hotels;

DROP FUNCTION IF EXISTS update_updated_at_column;

-- Drop Tables
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS bookings;
DROP TABLE IF EXISTS daily_rates;
DROP TABLE IF EXISTS rate_plans;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS room_types;
DROP TABLE IF EXISTS hotels;

-- Drop Types
DROP TYPE IF EXISTS payment_status;
DROP TYPE IF EXISTS booking_status;
DROP TYPE IF EXISTS operational_status;

-- Drop Extensions
DROP EXTENSION IF EXISTS "btree_gist";
DROP EXTENSION IF EXISTS "uuid-ossp";
