use crate::{
    database::DatabaseApp,
    entities::user_entity::{CreateUser, User},
};

#[derive(Clone)]
pub struct UsersRepository {
    database: DatabaseApp,
}

const USER_FIELDS: &str =
    "id, email, name, image, last_login_at, is_active, created_at, updated_at";

impl UsersRepository {
    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user: Option<User> =
            sqlx::query_as::<_, User>(&format!("SELECT {USER_FIELDS} FROM users WHERE email = $1"))
                .bind(&email)
                .fetch_optional(&self.database.pool)
                .await?;

        Ok(user)
    }

    pub async fn create_user_transaction(
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

        let user: User =
            sqlx::query_as::<_, User>(&format!("SELECT {USER_FIELDS} FROM users WHERE id = $1"))
                .bind(&id)
                .fetch_one(&mut *tx)
                .await?;

        tx.commit().await?;

        Ok(user)
    }
}
