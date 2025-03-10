use crate::{
    database::DatabaseApp,
    entities::auth_provider_entity::{AuthProvider, AuthProviderCreate},
};

#[derive(Clone)]
pub struct AuthProvidersRepository {
    database: DatabaseApp,
}

const AUTH_PROVIDERS_FIELDS: &str = "id, provider_type, created_at";

impl AuthProvidersRepository {
    pub fn new(database: DatabaseApp) -> Self {
        Self { database }
    }

    pub async fn create_auth_provider(
        &self,
        create_auth_provider: &AuthProviderCreate,
    ) -> Result<AuthProvider, sqlx::Error> {
        let mut tx = self.database.pool.begin().await?;

        println!("create_auth_provider: {:?}", create_auth_provider);

        sqlx::query("INSERT INTO auth_providers (id, provider_type) VALUES ($1, $2)")
            .bind(&create_auth_provider.id)
            .bind(&create_auth_provider.provider_type)
            .execute(&mut *tx)
            .await?;

        let auth_provider: AuthProvider = sqlx::query_as::<_, AuthProvider>(&format!(
            "SELECT {AUTH_PROVIDERS_FIELDS} FROM auth_providers WHERE id = $1"
        ))
        .bind(&create_auth_provider.id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(auth_provider)
    }
}
