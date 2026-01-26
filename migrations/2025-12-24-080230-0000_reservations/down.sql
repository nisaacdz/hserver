-- This file should undo anything in `up.sql`
DROP TABLE bookings_reservations;
DROP TABLE reservations;
DROP TYPE reservation_status;
DROP FUNCTION generate_reservation_code;
DROP EXTENSION citext;
