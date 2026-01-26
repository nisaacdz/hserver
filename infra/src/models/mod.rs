use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use derive_more::Display;
use diesel::prelude::*;
use std::collections::Bound;
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ReservationStatus"]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
    Expired,
    Refunded,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::TransactionStatus"]
pub enum TransactionStatus {
    Pending,
    Failed,
    Succeeded,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ExternalProvider"]
pub enum ExternalProvider {
    Stripe,
    Paystack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MaintenanceKind"]
pub enum MaintenanceKind {
    Electrical,
    Plumbing,
    Structural,
    Hvac,
    FireSafety,
    SecuritySystems,
    Groundskeeping,
    Janitorial,
    PestControl,
    ItNetwork,
    Painting,
    Appliances,
    OutOfService,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MaintenanceSeverity"]
pub enum MaintenanceSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MediaKind"]
pub enum MediaKind {
    Image,
    Video,
}

// =========================================================================
//  USERS & STAFF
// =========================================================================

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: Option<Uuid>,
    pub email: &'a str,
    pub password_hash: Option<&'a str>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = staff)]
pub struct Staff {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = staff)]
pub struct NewStaff {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
}

// =========================================================================
//  ROOMS & CLASSES
// =========================================================================

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = amenities)]
pub struct Amenity {
    pub id: Uuid,
    pub name: String,
    pub icon_key: Option<String>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = amenities)]
pub struct NewAmenity<'a> {
    pub id: Option<Uuid>,
    pub name: &'a str,
    pub icon_key: Option<&'a str>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = room_classes)]
pub struct RoomClass {
    pub id: Uuid,
    pub name: String,
    pub base_price: BigDecimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = room_classes)]
pub struct NewRoomClass<'a> {
    pub id: Option<Uuid>,
    pub name: &'a str,
    pub base_price: BigDecimal,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(RoomClass))]
#[diesel(belongs_to(Amenity))]
#[diesel(table_name = room_classes_amenities)]
#[diesel(primary_key(room_class_id, amenity_id))]
pub struct RoomClassAmenity {
    pub room_class_id: Uuid,
    pub amenity_id: Uuid,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(RoomClass, foreign_key = class_id))]
#[diesel(table_name = room_classes_media)]
pub struct RoomClassMedia {
    pub id: Uuid,
    pub class_id: Uuid,
    pub external_id: String,
    pub caption: Option<String>,
    pub kind: MediaKind,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = room_classes_media)]
pub struct NewRoomClassMedia {
    pub id: Option<Uuid>,
    pub class_id: Uuid,
    pub external_id: String,
    pub caption: Option<String>,
    pub kind: MediaKind,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(RoomClass, foreign_key = class_id))]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Room, foreign_key = room_id))]
#[diesel(table_name = rooms_media)]
pub struct RoomMedia {
    pub id: Uuid,
    pub room_id: Uuid,
    pub external_id: String,
    pub caption: Option<String>,
    pub kind: MediaKind,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = rooms_media)]
pub struct NewRoomMedia {
    pub id: Option<Uuid>,
    pub room_id: Uuid,
    pub external_id: String,
    pub caption: Option<String>,
    pub kind: MediaKind,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub id: Option<Uuid>,
    pub label: &'a str,
    pub class_id: Uuid,
}

// =========================================================================
//  BLOCKS
// =========================================================================

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Room))]
#[diesel(table_name = blocks)]
pub struct Block {
    pub id: Uuid,
    pub room_id: Uuid,
    pub interval: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = blocks)]
pub struct NewBlock {
    pub id: Option<Uuid>,
    pub room_id: Uuid,
    pub interval: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
}

// =========================================================================
//  RESERVATIONS & ADDONS
// =========================================================================

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(User, foreign_key = guest_id))]
#[diesel(table_name = reservations)]
pub struct Reservation {
    pub id: Uuid,
    pub code: String,
    pub guest_id: Uuid,
    pub status: ReservationStatus,
    pub total_amount: BigDecimal,
    pub currency: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = reservations)]
pub struct NewReservation {
    pub id: Option<Uuid>,
    // code generated by DB default
    pub guest_id: Uuid,
    pub status: ReservationStatus,
    pub total_amount: BigDecimal,
    pub currency: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = addons)]
pub struct Addon {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = addons)]
pub struct NewAddon {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Block))]
#[diesel(belongs_to(User, foreign_key = guest_id))]
#[diesel(table_name = bookings)]
#[diesel(primary_key(block_id))]
pub struct Booking {
    pub block_id: Uuid,
    pub guest_id: Uuid,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = bookings)]
pub struct NewBooking {
    pub block_id: Uuid,
    pub guest_id: Uuid,
}

#[derive(Queryable, Selectable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Booking))]
#[diesel(belongs_to(Addon))]
#[diesel(table_name = booking_addons)]
#[diesel(primary_key(booking_id, addon_id))]
pub struct BookingAddon {
    pub booking_id: Uuid,
    pub addon_id: Uuid,
    pub quantity: i32,
    pub price_at_booking: BigDecimal,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = booking_addons)]
pub struct NewBookingAddon {
    pub booking_id: Uuid,
    pub addon_id: Uuid,
    pub quantity: i32,
    pub price_at_booking: BigDecimal,
}

#[derive(Queryable, Selectable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Booking))]
#[diesel(belongs_to(Reservation))]
#[diesel(table_name = bookings_reservations)]
#[diesel(primary_key(booking_id, reservation_id))]
pub struct BookingReservation {
    pub booking_id: Uuid,
    pub reservation_id: Uuid,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = bookings_reservations)]
pub struct NewBookingReservation {
    pub booking_id: Uuid,
    pub reservation_id: Uuid,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Reservation))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub external_provider: ExternalProvider,
    pub external_id: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub status: TransactionStatus,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub id: Option<Uuid>,
    pub reservation_id: Uuid,
    pub external_provider: ExternalProvider,
    pub external_id: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub status: TransactionStatus,
    pub label: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Block))]
#[diesel(belongs_to(Staff, foreign_key = assigner_id))]
#[diesel(table_name = maintenance)]
#[diesel(primary_key(block_id))]
pub struct Maintenance {
    pub block_id: Uuid,
    pub kind: MaintenanceKind,
    pub severity: MaintenanceSeverity,
    pub assigner_id: Option<Uuid>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = maintenance)]
pub struct NewMaintenance {
    pub block_id: Uuid,
    pub kind: MaintenanceKind,
    pub severity: MaintenanceSeverity,
    pub assigner_id: Option<Uuid>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Block))]
#[diesel(table_name = reports)]
pub struct Report {
    pub id: Uuid,
    pub block_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = reports)]
pub struct NewReport<'a> {
    pub id: Option<Uuid>,
    pub block_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
}
