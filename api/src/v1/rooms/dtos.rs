use std::ops::Bound;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomAvailability {
    pub room_id: Uuid,
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    pub blocks: Vec<CalendarBlock>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBlock {
    pub id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(rename = "type")]
    pub kind: BlockKind,
    pub label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockKind {
    Booking,
    Maintenance,
    Unknown,
}
