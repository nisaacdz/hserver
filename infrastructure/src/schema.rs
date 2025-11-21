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
    use diesel::sql_types::*;
    use super::sql_types::BookingStatus;

    bookings (id) {
        id -> Uuid,
        code -> Text,
        hotel_id -> Uuid,
        room_id -> Uuid,
        guest_id -> Uuid,
        booking_period -> Tsrange,
        status -> BookingStatus,
        total_price -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    daily_rates (id) {
        id -> Uuid,
        hotel_id -> Uuid,
        room_type_id -> Uuid,
        rate_plan_id -> Uuid,
        date -> Date,
        price -> Numeric,
        currency -> Nullable<Bpchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    hotels (id) {
        id -> Uuid,
        name -> Text,
        address -> Nullable<Text>,
        timezone -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentStatus;

    payments (id) {
        id -> Uuid,
        booking_id -> Uuid,
        amount -> Numeric,
        status -> PaymentStatus,
        provider_txn_id -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    rate_plans (id) {
        id -> Uuid,
        hotel_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    room_types (id) {
        id -> Uuid,
        hotel_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        base_price -> Numeric,
        max_occupancy -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationalStatus;

    rooms (id) {
        id -> Uuid,
        room_type_id -> Uuid,
        hotel_id -> Uuid,
        room_number -> Text,
        operational_status -> OperationalStatus,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        full_name -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        role_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(bookings -> hotels (hotel_id));
diesel::joinable!(bookings -> rooms (room_id));
diesel::joinable!(bookings -> users (guest_id));
diesel::joinable!(daily_rates -> hotels (hotel_id));
diesel::joinable!(daily_rates -> rate_plans (rate_plan_id));
diesel::joinable!(daily_rates -> room_types (room_type_id));
diesel::joinable!(payments -> bookings (booking_id));
diesel::joinable!(rate_plans -> hotels (hotel_id));
diesel::joinable!(room_types -> hotels (hotel_id));
diesel::joinable!(rooms -> hotels (hotel_id));
diesel::joinable!(rooms -> room_types (room_type_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    daily_rates,
    hotels,
    payments,
    rate_plans,
    roles,
    room_types,
    rooms,
    users,
);
