use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateReservationRequest {
    pub items: Vec<ReservationItemRequest>,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct AddToCartRequest {
    pub reservation_id: Uuid,
    #[serde(flatten)]
    pub item: ReservationItemRequest,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReservationItemRequest {
    pub room_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    #[serde(default)]
    pub addon_ids: Vec<Uuid>,
}
