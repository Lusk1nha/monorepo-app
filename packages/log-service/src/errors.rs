use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogServiceError {
    #[error("Error creating logger")]
    FilterError,

    #[error("Error writing log: {0}")]
    SubscriberInit(String),
}
