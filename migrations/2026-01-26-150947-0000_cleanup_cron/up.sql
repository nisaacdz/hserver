-- Create extension if not exists
CREATE EXTENSION IF NOT EXISTS pg_cron;

-- Schedule the cleanup job to run every 2 minutes
SELECT cron.schedule('cleanup_expired_bookings', '*/2 * * * *', $$
    DELETE FROM blocks 
    WHERE id IN (
        SELECT block_id 
        FROM bookings 
        WHERE status = 'pending' 
        AND expires_at < NOW()
    );
$$);
