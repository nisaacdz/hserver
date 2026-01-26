use std::ops::Bound;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct BookSpecificRoomOptions {
    pub room_id: Uuid,
    pub interval: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    pub guest_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct BookAnyRoomOptions {
    pub guest_id: Uuid,
    pub interval: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
}

#[derive(Serialize)]
pub struct BookSuccess {
    pub booking_id: Uuid,
    pub status: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub amount: BigDecimal,
    pub currency: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum BookingError {
    RoomUnavailable,
    InternalError(String),
    ValidationError(String),
}
