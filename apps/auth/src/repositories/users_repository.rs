use crate::{
    database::DatabaseApp,
    entities::user_entity::{CreateUser, User},
};

#[derive(Clone)]
pub struct UsersRepository {
    database: DatabaseApp,
}

impl UsersRepository {
    const FIELDS: &'static str = "id, email, name, image, last_login_at, is_active, is_2fa_enabled, is_email_verified, otp_secret, created_at, updated_at";
    const TABLE: &'static str = "users";

    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user: Option<User> = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM {} WHERE email = $1",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&email)
        .fetch_optional(&self.database.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        id: &str,
        create_user: &CreateUser,
    ) -> Result<User, sqlx::Error> {
        let mut tx = self.database.pool.begin().await?;

        let user = sqlx::query_as::<_, User>(&format!(
            "INSERT INTO {} (id, email, name, image, otp_secret)
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&id)
        .bind(&create_user.email)
        .bind(&create_user.name)
        .bind(&create_user.image)
        .bind(&create_user.otp_secret)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn update_last_login_at(&self, user_id: &str) -> Result<User, sqlx::Error> {
        let mut tx = self.database.pool.begin().await?;

        let user: User = sqlx::query_as::<_, User>(&format!(
            "UPDATE {} SET last_login_at = now() WHERE id = $1 RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&user_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }
}
