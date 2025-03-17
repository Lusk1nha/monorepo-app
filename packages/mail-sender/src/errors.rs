use thiserror::Error;

#[derive(Error, Debug)]
pub enum MailServiceError {
    #[error("Error creating mailer: {0}")]
    CreateMailerError(String),

    #[error("Error sending mail: {0}")]
    SendMailError(String),

    #[error("Error getting template: {0}")]
    LoadTemplateError(#[from] MailLoadError),

    #[error("Error rendering template: {0}")]
    RenderTemplateError(#[from] MailRenderError),
}

#[derive(Error, Debug)]
pub enum MailRenderError {
    #[error("Error rendering mail template: {0}")]
    RenderTemplateError(#[from] tera::Error),

    #[error("Error template is empty")]
    EmptyTemplateError,
}

#[derive(Error, Debug)]

pub enum MailLoadError {
    #[error("Error loading mail template: {0}")]
    LoadTemplateError(String),
}
