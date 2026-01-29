use crate::reservations::{
    dtos::CreateReservationRequest, errors::ReservationError, types::ReservationCart,
};
use crate::settings::ReservationSettings;
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

/// Command for creating a reservation
#[derive(Debug, Clone)]
pub struct CreateReservationCommand {
    pub guest_id: Uuid,
    pub request: CreateReservationRequest,
}

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

/// Execute the create reservation command
///
/// This is the orchestrator that:
/// 1. Validates the request
/// 2. Calls the pricing calculator to determine costs
/// 3. Calls the infrastructure layer to persist data
/// 4. Returns a structured response
pub async fn execute<F, Fut>(
    command: CreateReservationCommand,
    settings: &ReservationSettings,
    persist_fn: F,
) -> Result<ReservationCreatedResponse, CreateReservationError>
where
    F: FnOnce(Uuid, CreateReservationRequest, &ReservationSettings) -> Fut,
    Fut: std::future::Future<Output = Result<ReservationCart, ReservationError>>,
{
    // Step 1: Validate request
    if command.request.items.is_empty() {
        return Err(CreateReservationError::ValidationError(
            "Reservation must have at least one item".to_string(),
        ));
    }

    // Step 2: Call infrastructure to persist
    // The pricing calculations are still being done in the infra layer
    // We'll refactor that in Phase 3
    let cart = persist_fn(command.guest_id, command.request, settings).await?;

    // Step 3: Build response
    let response = ReservationCreatedResponse {
        reservation_id: cart.id,
        code: cart.code,
        total_amount: cart.payable_amount,
        status: format!("{:?}", cart.status),
        expires_at: cart.expires_at,
    };

    Ok(response)
}
