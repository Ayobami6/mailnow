use super::{DbPool, UserRepository, UserRepositoryImpl, CoreRepository, CoreRepositoryImpl};

#[derive(Clone)]
pub struct RepositoryFactory {
    pool: DbPool,
}

impl RepositoryFactory {
    pub fn new(pool: DbPool) -> Self {
        log::info!("Creating repository factory");
        Self { pool }
    }

    pub fn create_user_repository(&self) -> UserRepositoryImpl {
        log::debug!("Creating user repository instance");
        UserRepositoryImpl::new(self.pool.clone())
    }

    pub fn create_core_repository(&self) -> impl CoreRepository {
        log::debug!("Creating core repository instance");
        CoreRepositoryImpl::new(self.pool.clone())
    }
}