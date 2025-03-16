use sqlx::QueryBuilder;

use crate::{
    database::DatabaseApp,
    entities::user_entity::{CreateUser, UpdateUser, User},
    errors::repository_errors::RepositoryError,
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

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        let user: Option<User> = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM {} WHERE id = $1",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&id)
        .fetch_optional(&self.database.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let user: Option<User> = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM {} WHERE email = $1",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&email)
        .fetch_optional(&self.database.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        id: &str,
        create_user: &CreateUser,
    ) -> Result<User, RepositoryError> {
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
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn update_user_properties(
        &self,
        user_id: &str,
        payload: UpdateUser,
    ) -> Result<User, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let mut query_builder = QueryBuilder::new(&format!("UPDATE {} SET ", Self::TABLE));

        let mut has_fields = false;

        if let Some(email) = payload.email {
            query_builder.push("email = ").push_bind(email);
            has_fields = true;
        }

        if let Some(name) = payload.name {
            query_builder.push("name = ").push_bind(name);
            has_fields = true;
        }

        if let Some(image) = payload.image {
            if has_fields {
                query_builder.push(", ");
            }

            query_builder.push("image = ").push_bind(image);
            has_fields = true;
        }

        if let Some(is_2fa_enabled) = payload.is_2fa_enabled {
            if has_fields {
                query_builder.push(", ");
            }

            query_builder
                .push("is_2fa_enabled = ")
                .push_bind(is_2fa_enabled);
            has_fields = true;
        }

        if let Some(is_email_verified) = payload.is_email_verified {
            if has_fields {
                query_builder.push(", ");
            }

            query_builder
                .push("is_email_verified = ")
                .push_bind(is_email_verified);

            has_fields = true;
        }

        if let Some(otp_secret) = payload.otp_secret {
            if has_fields {
                query_builder.push(", ");
            }

            query_builder.push("otp_secret = ").push_bind(otp_secret);
        }

        query_builder.push(" WHERE id = ").push_bind(user_id);

        println!("{}", query_builder.sql());
        let query_builder = query_builder.build();

        query_builder.execute(&mut *tx).await?;

        let user = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM {} WHERE id = $1",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn update_last_login_at(&self, user_id: &str) -> Result<User, RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        let user: User = sqlx::query_as::<_, User>(&format!(
            "UPDATE {} SET last_login_at = now() WHERE id = $1 RETURNING {}",
            Self::TABLE,
            Self::FIELDS
        ))
        .bind(&user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryError::UniqueViolation("id".into())
            }
            _ => RepositoryError::from(e),
        })?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), RepositoryError> {
        let mut tx = self.database.pool.begin().await?;

        sqlx::query(&format!("DELETE FROM {} WHERE id = $1", Self::TABLE))
            .bind(&user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                    RepositoryError::UniqueViolation("id".into())
                }
                _ => RepositoryError::from(e),
            })?;

        tx.commit().await?;

        Ok(())
    }
}
