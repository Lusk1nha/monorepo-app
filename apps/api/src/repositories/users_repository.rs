use crate::{
    database::DatabaseApp,
    entities::user_entity::{CreateUser, User},
};

#[derive(Clone)]
pub struct UsersRepository {
    database: DatabaseApp,
}

const USER_FIELDS: &str = "id, name, image, created_at, updated_at, last_login_at, is_active";

impl UsersRepository {
    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn create_user_transaction(
        &self,
        create_user: &CreateUser,
    ) -> Result<User, sqlx::Error> {
        let mut tx = self.database.pool.begin().await?;

        sqlx::query("INSERT INTO users (id, name, image) VALUES ($1, $2, $3)")
            .bind(&create_user.id)
            .bind(&create_user.name)
            .bind(&create_user.image)
            .execute(&mut *tx)
            .await?;

        let user: User =
            sqlx::query_as::<_, User>(&format!("SELECT {USER_FIELDS} FROM users WHERE id = $1"))
                .bind(&create_user.id)
                .fetch_one(&mut *tx)
                .await?;

        tx.commit().await?;

        Ok(user)
    }
}
