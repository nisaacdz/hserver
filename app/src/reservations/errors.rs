use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("Reservation session has expired")]
    SessionExpired {
        reservation_id: Uuid,
        expired_at: DateTime<Utc>,
    },

    #[error("The selected room is already booked for these dates")]
    RoomUnavailable {
        room_id: Uuid,
        // Return the conflicting interval so the frontend can gray it out
        conflict_start: DateTime<Utc>,
        conflict_end: DateTime<Utc>,
    },

    #[error("Minimum booking duration not met")]
    DurationTooShort { min_hours: i32 },

    #[error("Cannot modify a confirmed reservation")]
    ImmutableStatus { current_status: String },

    #[error("Room class '{0}' not found")]
    RoomClassNotFound(Uuid),

    #[error("Database error")]
    DbError(#[from] diesel::result::Error),

    // useful if you add capacity checks later
    #[error("Max capacity exceeded for this room class")]
    CapacityExceeded,

    #[error("Internal server error: {0}")]
    InternalError(String),
}
