-- This file should undo anything in `up.sql`

SELECT cron.unschedule('cleanup_expired_bookings');
DROP FUNCTION cleanup_expired_data;
DROP EXTENSION pg_cron;
