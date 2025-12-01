
INSERT INTO users (email, password_hash) VALUES
('admin@hotel.com', 'placeholder_hash_admin'),
('staff@hotel.com', 'placeholder_hash_staff'),
('guest@example.com', 'placeholder_hash_guest')
ON CONFLICT (email) DO NOTHING;

INSERT INTO room_classes (name, base_price) VALUES
('Standard', 100.00),
('Deluxe', 200.00),
('Suite', 350.00);

INSERT INTO amenities (name, icon_key) VALUES
('WiFi', 'wifi'),
('Air Conditioning', 'ac'),
('Pool', 'pool'),
('Breakfast', 'breakfast'),
('Gym', 'gym'),
('Parking', 'parking')
ON CONFLICT (name) DO NOTHING;

INSERT INTO room_classes_amenities (room_class_id, amenity_id)
SELECT rc.id, a.id 
FROM room_classes rc, amenities a
WHERE rc.name = 'Standard' AND a.name IN ('WiFi', 'Air Conditioning')
ON CONFLICT DO NOTHING;

INSERT INTO room_classes_amenities (room_class_id, amenity_id)
SELECT rc.id, a.id 
FROM room_classes rc, amenities a
WHERE rc.name = 'Deluxe' AND a.name IN ('WiFi', 'Air Conditioning', 'Breakfast', 'Parking')
ON CONFLICT DO NOTHING;

INSERT INTO room_classes_amenities (room_class_id, amenity_id)
SELECT rc.id, a.id 
FROM room_classes rc, amenities a
WHERE rc.name = 'Suite'
ON CONFLICT DO NOTHING;

INSERT INTO rooms (label, class_id)
SELECT '101', id FROM room_classes WHERE name = 'Standard';
INSERT INTO rooms (label, class_id)
SELECT '102', id FROM room_classes WHERE name = 'Standard';
INSERT INTO rooms (label, class_id)
SELECT '103', id FROM room_classes WHERE name = 'Standard';

INSERT INTO rooms (label, class_id)
SELECT '201', id FROM room_classes WHERE name = 'Deluxe';
INSERT INTO rooms (label, class_id)
SELECT '202', id FROM room_classes WHERE name = 'Deluxe';

INSERT INTO rooms (label, class_id)
SELECT '301', id FROM room_classes WHERE name = 'Suite';

INSERT INTO staff (user_id)
SELECT id FROM users WHERE email = 'staff@hotel.com';
WITH inserted_block AS (
    INSERT INTO blocks (room_id, interval)
    SELECT r.id, tstzrange(NOW() + INTERVAL '7 days', NOW() + INTERVAL '10 days')
    FROM rooms r
    WHERE r.label = '101'
    RETURNING id
)
INSERT INTO bookings (block_id, guest_id, status, payment_status)
SELECT ib.id, u.id, 'CONFIRMED', 'PAID'
FROM inserted_block ib, users u
WHERE u.email = 'guest@example.com';

