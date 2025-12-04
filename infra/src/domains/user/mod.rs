use crate::db::DbPool;
use crate::models::User as DbUser;
use crate::schema::users::dsl as users_dsl;
use app::api::ApiResponse;
use app::users::details::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn get_details(
    pool: &DbPool,
    options: GetUserDetailsOptions,
) -> ApiResponse<GetUserDetailsSuccess, GetUserDetailsError> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error(GetUserDetailsError::InternalError),
    };

    let user: DbUser = match users_dsl::users
        .find(options.user_id)
        .first(&mut conn)
        .await
    {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return ApiResponse::error(GetUserDetailsError::NotFound);
        }
        Err(_) => return ApiResponse::error(GetUserDetailsError::InternalError),
    };

    ApiResponse::success(GetUserDetailsSuccess {
        user: UserDetails {
            id: user.id,
            email: user.email,
        },
    })
}
