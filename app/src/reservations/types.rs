use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PA {
    pub percent: f64,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationCart {
    pub id: Uuid,
    pub code: String,
    pub status: ReservationStatus,
    pub currency: String,
    pub total_amount: f64,
    pub discount: PA,
    pub vat: PA,
    pub payable_amount: f64,
    pub expires_at: Option<DateTime<Utc>>,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub booking_id: Uuid,
    pub room_id: Uuid,

    pub room_label: String,
    pub room_class_name: String,

    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    pub item_total: f64,

    pub addons: Vec<CartAddon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartAddon {
    pub addon_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
    Expired,
}
