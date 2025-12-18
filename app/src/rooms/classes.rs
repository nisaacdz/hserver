use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomClassWithAmenities {
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String)]
    pub base_price: BigDecimal,
    pub amenities: Vec<Amenity>,
    pub media: Vec<Media>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: Uuid,
    pub url: String,
    pub caption: Option<String>,
    pub kind: MediaKind,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum MediaKind {
    Image,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Amenity {
    pub id: Uuid,
    pub name: String,
    pub icon_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum GetClassesError {
    InternalError,
}

impl Display for GetClassesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetClassesError::InternalError => write!(f, "Internal Server Error"),
        }
    }
}

impl ResponseError for GetClassesError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetClassesError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
