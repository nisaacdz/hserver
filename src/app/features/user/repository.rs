use super::entities::{NewUser, User};
use crate::error::AppError;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        use crate::schema::users;

        let mut conn = self.pool.get().await?;
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get().await?;
        let user = users.find(user_id).get_result::<User>(&mut conn).await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, user_email: &str) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get().await?;
        let user = users
            .filter(email.eq(user_email))
            .get_result::<User>(&mut conn)
            .await?;

        Ok(user)
    }

    pub async fn list_all(&self) -> Result<Vec<User>, AppError> {
        use crate::schema::users::dsl::*;

        let mut conn = self.pool.get().await?;
        let user_list = users.load::<User>(&mut conn).await?;

        Ok(user_list)
    }
}
