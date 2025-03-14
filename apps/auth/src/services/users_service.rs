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

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UsersError> {
        let user = self
            .users_repository
            .get_user_by_email(&email)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }

    pub async fn create_user(&self, email: &str) -> Result<User, UsersError> {
        let id = create_uuid_v4();

        let payload = CreateUser {
            email: email.into(),
            name: None,
            image: None,
        };

        let user = self
            .users_repository
            .create_user(&id, &payload)
            .await
            .map_err(UsersError::Database)?;

        Ok(user)
    }
}
