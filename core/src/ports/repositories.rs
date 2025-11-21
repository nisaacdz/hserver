use async_trait::async_trait;
use uuid::Uuid;
use crate::entities::user::User;
use crate::entities::booking::Booking;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Entity not found")]
    NotFound,
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>;
    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, RepositoryError>;
}

#[async_trait]
pub trait BookingRepository: Send + Sync {
    async fn create(&self, booking: Booking) -> Result<Booking, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Booking>, RepositoryError>;
    // Add more methods as needed
}
