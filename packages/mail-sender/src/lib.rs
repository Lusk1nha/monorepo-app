use std::{path::PathBuf, sync::Arc};

use errors::MailServiceError;
use lettre::{
    SmtpTransport,
    message::header::ContentType,
    transport::smtp::authentication::{Credentials, Mechanism},
};
use sender::EmailSender;
use tera::Context;
use tokio::sync::mpsc;
use tracing::{Level, debug, error, info, instrument, span};

mod build;
pub mod errors;
mod load;
mod render;
mod sender;

#[derive(Clone, Debug)]
pub struct SMTPConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[derive(Debug)]
pub struct EmailTask {
    pub request: EmailRequest,
}

#[derive(Debug)]
pub struct EmailRequest {
    pub from: String,
    pub to: String,
    pub header: Option<ContentType>,
    pub subject: String,
    pub template_name: String,
    pub context: Option<Context>,
}

/// # Mail Service
/// The mail service is a simple wrapper around the `lettre` crate.
/// It provides a simple way to send emails using an SMTP server.
/// The service is initialized with an `SMTPConfig` struct that contains the following fields:
/// - `smtp_server`: The address of the SMTP server.
/// - `smtp_port`: The port of the SMTP server.
/// - `smtp_username`: The username for the SMTP server.
/// - `smtp_password`: The password for the SMTP server.
/// The service can be used to queue email requests for sending.
/// The email requests contain the following fields:
/// - `from`: The sender's email address.
/// - `to`: The recipient's email address.
/// - `header`: The content type of the email.
/// - `subject`: The subject of the email.
/// - `template_name`: The name of the email template.
/// - `context`: The context data for rendering the email template.
/// The service uses a worker task to process email requests asynchronously.
/// The worker task sends the email using the configured SMTP server.
/// The service also provides a method to queue email requests for sending.
/// The email requests are processed in a first-in-first-out (FIFO) order.
/// The service logs errors and information messages using the `tracing` crate.
/// The service can be initialized with the default configuration by calling `MailService::new()`.
/// The service can be customized by providing an optional template directory.
pub struct MailService {
    sender: mpsc::Sender<EmailTask>,
    #[allow(dead_code)]
    email_sender: EmailSender,
}

impl MailService {
    #[instrument(name = "MailService::new", skip(config, template_dir))]
    pub async fn new(
        config: SMTPConfig,
        template_dir: Option<PathBuf>,
        queue_capacity: usize,
    ) -> Result<Self, MailServiceError> {
        info!("Initializing MailService");

        let mailer = Self::instance_mailer(config).await?;
        debug!("SMTP mailer instance created successfully");

        let template_dir = template_dir.unwrap_or_else(|| {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            current_dir.join("templates")
        });
        debug!(?template_dir, "Template directory resolved");

        let email_sender = EmailSender::new(Arc::clone(&mailer), template_dir.clone());
        debug!("EmailSender instance created");

        let (sender, receiver) = mpsc::channel(queue_capacity);
        debug!("Channel created with capacity: {}", queue_capacity);

        let worker_sender = email_sender.clone();
        tokio::spawn(Self::email_worker(worker_sender, receiver));
        info!("Email worker spawned");

        Ok(Self {
            sender,
            email_sender,
        })
    }

    #[instrument(name = "MailService::instance_mailer", skip(config))]
    async fn instance_mailer(config: SMTPConfig) -> Result<Arc<SmtpTransport>, MailServiceError> {
        info!("Creating SMTP mailer instance");
        let credentials =
            Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

        let mailer = SmtpTransport::relay(&config.smtp_server)
            .map_err(|e| {
                error!("Failed to create SMTP relay: {}", e);
                MailServiceError::CreateMailerError(e.to_string())
            })?
            .credentials(credentials)
            .port(config.smtp_port)
            .authentication(vec![Mechanism::Plain])
            .build();

        debug!("SMTP mailer configured successfully");
        Ok(Arc::new(mailer))
    }

    #[instrument(name = "MailService::email_worker", skip(email_sender, receiver))]
    async fn email_worker(email_sender: EmailSender, mut receiver: mpsc::Receiver<EmailTask>) {
        info!("Email worker started");
        while let Some(task) = receiver.recv().await {
            let span = span!(Level::INFO, "process_email_task", task = ?task);
            let _enter = span.enter();

            let sender = email_sender.clone();
            tokio::spawn(async move {
                if let Err(e) = sender.send(task.request).await {
                    error!("Error sending email: {}", e);
                } else {
                    debug!("Email sent successfully");
                }
            });
        }
        info!("Email worker shutting down");
    }

    #[instrument(name = "MailService::queue_email", skip(self, request))]
    pub async fn queue_email(&self, request: EmailRequest) -> Result<(), MailServiceError> {
        debug!("Queueing email request");
        let task = EmailTask { request };
        self.sender.send(task).await.map_err(|e| {
            error!("Failed to queue email: {}", e);
            MailServiceError::QueueError(e.to_string())
        })?;
        debug!("Email queued successfully");
        Ok(())
    }
}
