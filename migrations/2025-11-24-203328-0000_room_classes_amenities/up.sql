-- Your SQL goes here

CREATE TABLE room_classes_amenities (
    room_class_id UUID NOT NULL REFERENCES room_classes(id) ON DELETE CASCADE,
    amenity_id UUID NOT NULL REFERENCES amenities(id) ON DELETE CASCADE,
    PRIMARY KEY (room_class_id, amenity_id)
);
