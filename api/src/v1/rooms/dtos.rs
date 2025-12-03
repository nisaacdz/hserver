use std::ops::Bound;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct RoomAvailabilityQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomAvailability {
    pub room_id: Uuid,
    #[schema(value_type = Vec<String>, example = json!(["2023-01-01T00:00:00Z", "2023-01-02T00:00:00Z"]))]
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    pub blocks: Vec<CalendarBlock>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBlock {
    pub id: Uuid,
    #[schema(value_type = Vec<String>)]
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    #[serde(rename = "type")]
    pub kind: BlockKind,
    pub label: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockKind {
    Booking,
    Maintenance,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AmenityDto {
    pub id: Uuid,
    pub name: String,
    pub icon_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoomClassResponse {
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String)]
    pub base_price: bigdecimal::BigDecimal,
    pub amenities: Vec<AmenityDto>,
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct FindRoomQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub class_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomSummary {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct FindRoomResponse {
    pub rooms: Vec<RoomSummary>,
}
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomClassSummaryDto {
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String)]
    pub base_price: bigdecimal::BigDecimal,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailsDto {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
    pub class: RoomClassSummaryDto,
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct RoomListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomDto {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomListResponse {
    pub rooms: Vec<RoomDto>,
    pub total_rooms: i64,
    pub page: i64,
}
