use std::ops::Bound;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RoomAvailabilityQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomAvailability {
    pub room_id: Uuid,
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    pub blocks: Vec<CalendarBlock>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBlock {
    pub id: Uuid,
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    #[serde(rename = "type")]
    pub kind: BlockKind,
    pub label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockKind {
    Booking,
    Maintenance,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmenityDto {
    pub id: Uuid,
    pub name: String,
    pub icon_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomClassResponse {
    pub id: Uuid,
    pub name: String,
    pub base_price: bigdecimal::BigDecimal,
    pub amenities: Vec<AmenityDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindRoomQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub class_id: Option<Uuid>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSummary {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
}

#[derive(Serialize)]
pub struct FindRoomResponse {
    pub rooms: Vec<RoomSummary>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomClassSummaryDto {
    pub id: Uuid,
    pub name: String,
    pub base_price: bigdecimal::BigDecimal,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailsDto {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
    pub class: RoomClassSummaryDto,
}
