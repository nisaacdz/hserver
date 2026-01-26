// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "external_provider"))]
    pub struct ExternalProvider;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "maintenance_kind"))]
    pub struct MaintenanceKind;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "maintenance_severity"))]
    pub struct MaintenanceSeverity;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "media_kind"))]
    pub struct MediaKind;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "reservation_status"))]
    pub struct ReservationStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_status"))]
    pub struct TransactionStatus;
}

diesel::table! {
    addons (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        price -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    amenities (id) {
        id -> Uuid,
        name -> Text,
        icon_key -> Nullable<Text>,
    }
}

diesel::table! {
    blocks (id) {
        id -> Uuid,
        room_id -> Uuid,
        interval -> Tstzrange,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    booking_addons (booking_id, addon_id) {
        booking_id -> Uuid,
        addon_id -> Uuid,
        quantity -> Int4,
        price_at_booking -> Numeric,
    }
}

diesel::table! {
    bookings (block_id) {
        block_id -> Uuid,
        guest_id -> Uuid,
    }
}

diesel::table! {
    bookings_reservations (booking_id, reservation_id) {
        booking_id -> Uuid,
        reservation_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MaintenanceKind;
    use super::sql_types::MaintenanceSeverity;

    maintenance (block_id) {
        block_id -> Uuid,
        kind -> MaintenanceKind,
        severity -> MaintenanceSeverity,
        assigner_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    otps (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 24]
        code -> Varchar,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reports (id) {
        id -> Uuid,
        block_id -> Uuid,
        title -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReservationStatus;

    reservations (id) {
        id -> Uuid,
        code -> Citext,
        guest_id -> Uuid,
        status -> ReservationStatus,
        total_amount -> Numeric,
        currency -> Text,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    room_classes (id) {
        id -> Uuid,
        name -> Text,
        base_price -> Numeric,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    room_classes_amenities (room_class_id, amenity_id) {
        room_class_id -> Uuid,
        amenity_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaKind;

    room_classes_media (id) {
        id -> Uuid,
        class_id -> Uuid,
        external_id -> Text,
        caption -> Nullable<Text>,
        kind -> MediaKind,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    rooms (id) {
        id -> Uuid,
        #[max_length = 55]
        label -> Varchar,
        class_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaKind;

    rooms_media (id) {
        id -> Uuid,
        room_id -> Uuid,
        external_id -> Text,
        caption -> Nullable<Text>,
        kind -> MediaKind,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    staff (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ExternalProvider;
    use super::sql_types::TransactionStatus;

    transactions (id) {
        id -> Uuid,
        reservation_id -> Uuid,
        external_provider -> ExternalProvider,
        external_id -> Text,
        amount -> Numeric,
        currency -> Text,
        status -> TransactionStatus,
        label -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(blocks -> rooms (room_id));
diesel::joinable!(booking_addons -> addons (addon_id));
diesel::joinable!(booking_addons -> bookings (booking_id));
diesel::joinable!(bookings -> blocks (block_id));
diesel::joinable!(bookings -> users (guest_id));
diesel::joinable!(bookings_reservations -> bookings (booking_id));
diesel::joinable!(bookings_reservations -> reservations (reservation_id));
diesel::joinable!(maintenance -> blocks (block_id));
diesel::joinable!(maintenance -> staff (assigner_id));
diesel::joinable!(otps -> users (user_id));
diesel::joinable!(reservations -> users (guest_id));
diesel::joinable!(room_classes_amenities -> amenities (amenity_id));
diesel::joinable!(room_classes_amenities -> room_classes (room_class_id));
diesel::joinable!(room_classes_media -> room_classes (class_id));
diesel::joinable!(rooms -> room_classes (class_id));
diesel::joinable!(rooms_media -> rooms (room_id));
diesel::joinable!(staff -> users (user_id));
diesel::joinable!(transactions -> reservations (reservation_id));

diesel::allow_tables_to_appear_in_same_query!(
    addons,
    amenities,
    blocks,
    booking_addons,
    bookings,
    bookings_reservations,
    maintenance,
    otps,
    reports,
    reservations,
    room_classes,
    room_classes_amenities,
    room_classes_media,
    rooms,
    rooms_media,
    staff,
    transactions,
    users,
);
