use crate::{
    entities::user_entity::{CreateUser, User},
    errors::users_errors::UsersError,
    repositories::users_repository::UsersRepository,
    utils::uuid::create_uuid_v4,
};

#[derive(Clone)]
pub struct UsersService {
    pub users_repository: UsersRepository,
}

impl UsersService {
    pub fn new(users_repository: UsersRepository) -> Self {
        Self { users_repository }
    }

    pub async fn create_user_credentials(&self) -> Result<User, UsersError> {
        let uuid = create_uuid_v4();

        let create_user_payload = CreateUser {
            id: uuid,
            name: None,
            image: None,
        };

        let user = self
            .users_repository
            .create_user_transaction(&create_user_payload)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }

    pub async fn create_user_provider(&self, provider_id: &str) -> Result<User, UsersError> {
        let create_user_payload = CreateUser {
            id: provider_id.to_string(),
            name: None,
            image: None,
        };

        let user = self
            .users_repository
            .create_user_transaction(&create_user_payload)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }
}
