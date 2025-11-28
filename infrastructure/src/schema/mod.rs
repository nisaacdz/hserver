// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "maintenance_kind"))]
    pub struct MaintenanceKind;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "maintenance_severity"))]
    pub struct MaintenanceSeverity;
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
    bookings (block_id) {
        block_id -> Uuid,
        guest_id -> Uuid,
        status -> Text,
        payment_status -> Text,
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
    rooms (id) {
        id -> Uuid,
        #[max_length = 55]
        label -> Varchar,
        class_id -> Uuid,
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
    users (id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(blocks -> rooms (room_id));
diesel::joinable!(bookings -> blocks (block_id));
diesel::joinable!(bookings -> users (guest_id));
diesel::joinable!(maintenance -> blocks (block_id));
diesel::joinable!(maintenance -> staff (assigner_id));
diesel::joinable!(room_classes_amenities -> amenities (amenity_id));
diesel::joinable!(room_classes_amenities -> room_classes (room_class_id));
diesel::joinable!(rooms -> room_classes (class_id));
diesel::joinable!(staff -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    amenities,
    blocks,
    bookings,
    maintenance,
    reports,
    room_classes,
    room_classes_amenities,
    rooms,
    staff,
    users,
);
