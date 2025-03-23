use thiserror::Error;

#[derive(Error, Debug)]
pub enum MailServiceError {
    #[error("Error setting up service: {0}")]
    SetupService(String),

    #[error("Error creating mailer: {0}")]
    CreateMailerError(String),

    #[error("Error sending mail: {0}")]
    SendMailError(String),

    #[error("Error getting template: {0}")]
    LoadTemplateError(#[from] MailLoadError),

    #[error("Error rendering template: {0}")]
    RenderTemplateError(#[from] MailRenderError),

    #[error("Error queueing email: {0}")]
    QueueError(String),
}

#[derive(Error, Debug)]
pub enum MailBuilderError {
    #[error("Error building template: {0}")]
    BuildTemplateError(#[from] MailLoadError),

    #[error("Error rendering template: {0}")]
    RenderTemplateError(#[from] MailRenderError),
}

#[derive(Error, Debug)]
pub enum MailSendError {
    #[error("Error sending mail: {0}")]
    SendMailError(String),

    #[error("Error building mail: {0}")]
    BuildMessageError(String),
    
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
