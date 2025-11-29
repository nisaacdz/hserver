use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FindRoomQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub class_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct RoomSummary {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
}

#[derive(Serialize)]
pub struct FindRoomResponse {
    pub rooms: Vec<RoomSummary>,
}
