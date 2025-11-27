use std::rc::Rc;

use actix_web::{HttpResponse, web};
use infrastructure::db::DbPool;
use uuid::Uuid;

use crate::{auth::SessionUser, errors::LoginError};

/// Returns a structure with a vec of intervals (with details on each interval), blocking or
pub async fn availability(
    _pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, LoginError> {
    let class_id = path.into_inner();
    dbg!(user, class_id);
    todo!()
}

pub async fn details(
    _pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, LoginError> {
    let room_id = path.into_inner();
    dbg!(user, room_id);
    todo!()
}
