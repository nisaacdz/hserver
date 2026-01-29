use crate::reservations::{
    dtos::CreateReservationRequest, errors::ReservationError, types::ReservationCart,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

/// Success response when reservation is created
#[derive(Debug, Serialize, ToSchema)]
pub struct ReservationCreatedResponse {
    pub reservation_id: Uuid,
    pub code: String,
    pub total_amount: f64,
    pub status: String,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Errors that can occur during reservation creation
#[derive(Debug, Error)]
pub enum CreateReservationError {
    #[error("Room {0} is unavailable")]
    RoomUnavailable(Uuid),

    #[error("Invalid reservation request: {0}")]
    ValidationError(String),

    #[error("Infrastructure error: {0}")]
    InfraError(#[from] ReservationError),
}

impl actix_web::ResponseError for CreateReservationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CreateReservationError::RoomUnavailable(_) => {
                actix_web::http::StatusCode::CONFLICT
            }
            CreateReservationError::ValidationError(_) => {
                actix_web::http::StatusCode::BAD_REQUEST
            }
            CreateReservationError::InfraError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

/// Validate reservation request
pub fn validate_request(request: &CreateReservationRequest) -> Result<(), CreateReservationError> {
    if request.items.is_empty() {
        return Err(CreateReservationError::ValidationError(
            "Reservation must have at least one item".to_string(),
        ));
    }
    Ok(())
}

/// Build response from reservation cart
pub fn build_response(cart: ReservationCart) -> ReservationCreatedResponse {
    ReservationCreatedResponse {
        reservation_id: cart.id,
        code: cart.code,
        total_amount: cart.payable_amount,
        status: format!("{:?}", cart.status),
        expires_at: cart.expires_at,
    }
}
