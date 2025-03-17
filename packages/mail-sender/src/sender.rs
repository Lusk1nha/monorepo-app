use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use std::{path::PathBuf, sync::Arc};

use crate::{EmailRequest, build::EmailBuilder, errors::MailSendError};

#[derive(Clone)]
pub struct EmailSender {
    mailer: Arc<SmtpTransport>,
    builder: EmailBuilder,
}

impl EmailSender {
    pub fn new(mailer: Arc<SmtpTransport>, template_dir: PathBuf) -> Self {
        let builder = EmailBuilder::new(template_dir);

        Self { mailer, builder }
    }

    pub async fn send(&self, request: EmailRequest) -> Result<(), MailSendError> {
        let mailer = Arc::clone(&self.mailer);
        let message = self.build_message(request)?;

        tokio::task::spawn_blocking(move || mailer.send(&message))
            .await
            .map_err(|e| MailSendError::SendMailError(e.to_string()))?
            .map_err(|e| MailSendError::SendMailError(e.to_string()))?;

        Ok(())
    }

    fn build_message(&self, request: EmailRequest) -> Result<Message, MailSendError> {
        let body = self
            .builder
            .build_template(&request.template_name, request.context)
            .map_err(|e| MailSendError::BuildMessageError(e.to_string()))?;

        let message = Message::builder()
            .from(request.from.parse().map_err(|e| {
                MailSendError::BuildMessageError(format!("Invalid 'from' address: {}", e))
            })?)
            .to(request.to.parse().map_err(|e| {
                MailSendError::BuildMessageError(format!("Invalid 'to' address: {}", e))
            })?)
            .header(request.header.unwrap_or(ContentType::TEXT_PLAIN))
            .subject(request.subject)
            .body(body)
            .map_err(|e| MailSendError::BuildMessageError(e.to_string()))?;

        Ok(message)
    }
}
