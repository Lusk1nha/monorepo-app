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

pub struct MailService {
    sender: mpsc::Sender<EmailTask>,

    #[allow(dead_code)]
    email_sender: EmailSender,
}

impl MailService {
    pub async fn new(
        config: SMTPConfig,
        template_dir: Option<PathBuf>,
        queue_capacity: usize,
    ) -> Result<Self, MailServiceError> {
        let mailer = Self::instance_mailer(config).await?;

        let template_dir = template_dir.unwrap_or_else(|| {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            current_dir.join("templates")
        });

        let email_sender = EmailSender::new(Arc::clone(&mailer), template_dir.clone());

        let (sender, receiver) = mpsc::channel(queue_capacity);

        let worker_sender = email_sender.clone();
        tokio::spawn(Self::email_worker(worker_sender, receiver));

        Ok(Self {
            sender,

            email_sender,
        })
    }

    async fn instance_mailer(config: SMTPConfig) -> Result<Arc<SmtpTransport>, MailServiceError> {
        let credentials =
            Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

        let mailer = SmtpTransport::relay(&config.smtp_server)
            .map_err(|e| MailServiceError::CreateMailerError(e.to_string()))?
            .credentials(credentials)
            .port(config.smtp_port)
            .authentication(vec![Mechanism::Plain])
            .build();

        Ok(Arc::new(mailer))
    }

    async fn email_worker(email_sender: EmailSender, mut receiver: mpsc::Receiver<EmailTask>) {
        while let Some(task) = receiver.recv().await {
            let sender = email_sender.clone();
            tokio::spawn(async move {
                if let Err(e) = sender.send(task.request).await {
                    eprintln!("Error sending email: {}", e);
                }
            });
        }
    }

    pub async fn queue_email(&self, request: EmailRequest) -> Result<(), MailServiceError> {
        let task = EmailTask { request };
        self.sender
            .send(task)
            .await
            .map_err(|e| MailServiceError::QueueError(e.to_string()))?;
        Ok(())
    }
}
