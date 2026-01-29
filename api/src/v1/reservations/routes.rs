use actix_web::{web, HttpResponse};
use app::reservations::{create::CreateReservationError, CreateReservationRequest};
use app::AppSettings;
use infra::db::DbPool;
use infra::domains::reservations;
use std::rc::Rc;

use crate::auth::SessionUser;

#[utoipa::path(
    post,
    path = "/api/v1/reservations",
    request_body = CreateReservationRequest,
    responses(
        (status = 201, description = "Reservation created successfully"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Room unavailable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_reservation(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    json: web::Json<CreateReservationRequest>,
    settings: web::Data<AppSettings>,
) -> Result<HttpResponse, CreateReservationError> {
    let request = json.into_inner();

    // Validate request
    app::reservations::create::validate_request(&request)?;

    // Call the infrastructure layer to persist and calculate
    let cart = reservations::create::create_reservation(
        &pool,
        user.id,
        request,
        &settings.reservation,
    )
    .await?;

    // Build response
    let response = app::reservations::create::build_response(cart);

    Ok(HttpResponse::Created().json(response))
}

pub fn configure_reservations_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservations")
            .route("", web::post().to(create_reservation)),
    );
}
