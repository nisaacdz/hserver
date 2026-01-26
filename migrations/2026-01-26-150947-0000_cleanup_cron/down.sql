-- Unschedule the job
SELECT cron.unschedule('cleanup_expired_bookings');

DROP EXTENSION IF EXISTS pg_cron;
