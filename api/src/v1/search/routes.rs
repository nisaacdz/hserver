use actix_web::{web, HttpResponse};
use infrastructure::db::DbPool;

use crate::{dtos::FindRoom, errors::LoginError};

pub async fn find_room(
    _pool: web::Data<DbPool>,
    web::Query(FindRoom { class_id, period }): web::Query<FindRoom>,
) -> Result<HttpResponse, LoginError> {
    dbg!(class_id, period);
    todo!()
}
