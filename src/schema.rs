// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "booking_status"))]
    pub struct BookingStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "operational_status"))]
    pub struct OperationalStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_status"))]
    pub struct PaymentStatus;
}

diesel::table! {
    booking_services (booking_id, service_id) {
        booking_id -> Int4,
        service_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BookingStatus;

    bookings (booking_id) {
        booking_id -> Int4,
        guest_id -> Int4,
        room_id -> Int4,
        checkin_date -> Date,
        checkout_date -> Date,
        total_price -> Numeric,
        booking_status -> BookingStatus,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    guests (guest_id) {
        guest_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        hashed_password -> Nullable<Varchar>,
    }
}

diesel::table! {
    hotels (hotel_id) {
        hotel_id -> Int4,
        name -> Varchar,
        address -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentStatus;

    payments (payment_id) {
        payment_id -> Int4,
        booking_id -> Int4,
        amount -> Numeric,
        payment_status -> PaymentStatus,
        provider_txn_id -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (permission_id) {
        permission_id -> Int4,
        permission_name -> Varchar,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Int4,
        role_name -> Varchar,
    }
}

diesel::table! {
    room_types (room_type_id) {
        room_type_id -> Int4,
        hotel_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        base_price -> Numeric,
        max_occupancy -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationalStatus;

    rooms (room_id) {
        room_id -> Int4,
        room_type_id -> Int4,
        hotel_id -> Int4,
        room_number -> Varchar,
        operational_status -> OperationalStatus,
    }
}

diesel::table! {
    services (service_id) {
        service_id -> Int4,
        name -> Varchar,
        price -> Numeric,
    }
}

diesel::table! {
    staff (staff_id) {
        staff_id -> Int4,
        username -> Varchar,
        hashed_password -> Varchar,
        role_id -> Int4,
    }
}

diesel::joinable!(booking_services -> bookings (booking_id));
diesel::joinable!(booking_services -> services (service_id));
diesel::joinable!(bookings -> guests (guest_id));
diesel::joinable!(bookings -> rooms (room_id));
diesel::joinable!(payments -> bookings (booking_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(room_types -> hotels (hotel_id));
diesel::joinable!(rooms -> hotels (hotel_id));
diesel::joinable!(rooms -> room_types (room_type_id));
diesel::joinable!(staff -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    booking_services,
    bookings,
    guests,
    hotels,
    payments,
    permissions,
    role_permissions,
    roles,
    room_types,
    rooms,
    services,
    staff,
);
