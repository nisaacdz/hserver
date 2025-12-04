use crate::v1;
use app::auth::SessionUser;
use app::auth::login::LoginRequest;
use app::auth::onboard::OnboardRequest;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        v1::auth::routes::login,
        v1::auth::routes::onboard,
        // Users
        v1::users::routes::get_user,
        // Rooms
        v1::rooms::routes::get_room_availability,
        v1::rooms::routes::get_room_details,
        v1::rooms::routes::get_room_classes,
        v1::rooms::routes::find_room,
    ),
    components(
        schemas(
            LoginRequest,
            OnboardRequest,
            SessionUser,
            v1::rooms::dtos::RoomAvailability,
            v1::rooms::dtos::CalendarBlock,
            v1::rooms::dtos::BlockKind,
            v1::rooms::dtos::AmenityDto,
            v1::rooms::dtos::RoomClassResponse,
            v1::rooms::dtos::RoomSummary,
            v1::rooms::dtos::FindRoomResponse,
            v1::rooms::dtos::RoomClassSummaryDto,
            v1::rooms::dtos::RoomDetailsDto,
        )
    ),
    tags(
        (name = "hserver", description = "HServer API")
    )
)]
pub struct ApiDoc;
