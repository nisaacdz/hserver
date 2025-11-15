use crate::app::features::user::repository::UserRepository;
use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {
    pub user_repository: UserRepository,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        let user_repository = UserRepository::new(pool.clone());

        Self {
            user_repository,
        }
    }
}
