// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Uuid,
        room_id -> Uuid,
        guest_id -> Uuid,
        stay_period -> Tsrange,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    rooms (id) {
        id -> Uuid,
        number -> Text,
        room_type -> Text,
        price_per_night -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Text,
        role -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(bookings -> rooms (room_id));
diesel::joinable!(bookings -> users (guest_id));

diesel::allow_tables_to_appear_in_same_query!(bookings, rooms, users,);
