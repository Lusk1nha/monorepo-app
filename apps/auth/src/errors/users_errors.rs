use thiserror::Error;

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
