-- Your SQL goes here

CREATE OR REPLACE FUNCTION cleanup_expired_bookings() RETURNS void AS $$
BEGIN
    WITH expired_rows AS (
        SELECT id 
        FROM reservations 
        WHERE status = 'pending' 
        AND expires_at < NOW()
        FOR UPDATE SKIP LOCKED
    ),
    blocks_to_kill AS (
        SELECT booking_id 
        FROM bookings_reservations 
        WHERE reservation_id IN (SELECT id FROM expired_rows)
    ),
    kill_blocks AS (
        DELETE FROM blocks 
        WHERE id IN (SELECT booking_id FROM blocks_to_kill)
    )
    UPDATE reservations
    SET status = 'expired'
    WHERE id IN (SELECT id FROM expired_rows);
END;
$$ LANGUAGE plpgsql;

SELECT cron.schedule('cleanup_expired_bookings', '*/2 * * * *', 'SELECT cleanup_expired_bookings()');
