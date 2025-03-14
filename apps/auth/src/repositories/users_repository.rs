use crate::{
    database::DatabaseApp,
    entities::user_entity::{CreateUser, User},
};

#[derive(Clone)]
pub struct UsersRepository {
    database: DatabaseApp,
}

impl UsersRepository {
    const FIELDS: &'static str =
        "id, email, name, image, last_login_at, is_active, created_at, updated_at";
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

        sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2)")
            .bind(&id)
            .bind(&create_user.email)
            .execute(&mut *tx)
            .await?;

        let user: User = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM {} WHERE id = $1",
            Self::FIELDS,
            Self::TABLE
        ))
        .bind(&id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }
}
