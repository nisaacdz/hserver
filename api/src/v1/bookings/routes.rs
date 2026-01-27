use actix_web::{HttpResponse, web};
use infra::db::DbPool;
use std::rc::Rc;
use std::ops::Bound;
use app::AppSettings;

use crate::auth::SessionUser;
use app::bookings::{CreateBookingRequest, CreateBookingResponse};
use infra::domains::booking::{self, CreateBookingOptions};

#[utoipa::path(
    post,
    path = "/api/v1/bookings",
    request_body = CreateBookingRequest,
    responses(
        (status = 201, description = "Booking created successfully", body = CreateBookingResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Room unavailable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_booking(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    json: web::Json<CreateBookingRequest>,
    settings: web::Data<AppSettings>,
) -> Result<HttpResponse, actix_web::Error> {
    let req = json.into_inner();

    if req.start >= req.end {
        return Ok(HttpResponse::BadRequest().body("Invalid time range"));
    }

    let options = CreateBookingOptions {
        room_id: req.room_id,
        interval: (Bound::Included(req.start), Bound::Excluded(req.end)),
        user_id: user.id,
        transaction_amount: req.amount,
        transaction_currency: req.currency,
        transaction_external_id: req.transaction_external_id,
    };

    match booking::create_booking(&pool, options, &settings.service).await {
        Ok(block_id) => Ok(HttpResponse::Created().json(CreateBookingResponse {
            booking_id: block_id,
            status: "pending".to_string(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::minutes(settings.service.booking_grace_duration_minutes as i64)), 
        })),
        Err(booking::CreateBookingError::RoomUnavailable) => {
            Ok(HttpResponse::Conflict().body("Room is unavailable for the selected dates"))
        }
        Err(booking::CreateBookingError::InternalError(e)) => {
            log::error!("Booking creation logic error: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
