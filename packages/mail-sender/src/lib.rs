use std::{path::PathBuf, sync::Arc};

use errors::MailServiceError;
use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::authentication::{Credentials, Mechanism},
};
use load::load_template;
use render::render_template;

use tera::Context;

mod errors;
mod load;
mod render;

#[derive(Clone, Debug)]
pub struct SMTPConfig {
    pub smtp_server: String,
    pub smtp_port: u16,

    pub smtp_username: String,
    pub smtp_password: String,
}

pub struct MailService {
    pub mailer: Arc<SmtpTransport>,
    template_dir: PathBuf,
}

impl MailService {
    pub async fn new(
        config: SMTPConfig,
        template_dir: Option<PathBuf>,
    ) -> Result<Self, MailServiceError> {
        let mailer = Self::instance_mailer(config).await?;

        let template_dir = template_dir.unwrap_or_else(|| {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            current_dir.join("templates")
        });

        Ok(Self {
            mailer,
            template_dir,
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

    pub fn build_template_by_name(&self, name: &str) -> Result<String, MailServiceError> {
        let template = load_template(&self.template_dir, name)?;
        let render = render_template(&template, Context::new())?;
        Ok(render)
    }

    pub fn build_template_by_name_with_context(
        &self,
        name: &str,
        context: Context,
    ) -> Result<String, MailServiceError> {
        let template = load_template(&self.template_dir, name)?;

        let render = render_template(&template, context)?;

        Ok(render)
    }

    pub async fn send_mail(&self, message: Message) -> Result<(), MailServiceError> {
        let mailer = Arc::clone(&self.mailer);

        tokio::task::spawn_blocking(move || mailer.send(&message))
            .await
            .map_err(|e| MailServiceError::SendMailError(e.to_string()))?
            .map_err(|e| MailServiceError::SendMailError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use tera::Context;

    #[tokio::test]
    async fn test_get_template_by_name() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("test_template.html");
        let mut file = File::create(&template_path).unwrap();
        writeln!(file, "<h1>Hello, World!</h1>").unwrap();

        let config = SMTPConfig {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            smtp_username: "user".to_string(),
            smtp_password: "password".to_string(),
        };
        let mail_service = MailService::new(config, Some(dir.path().to_path_buf()))
            .await
            .unwrap();

        let result = mail_service.build_template_by_name("test_template.html");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "<h1>Hello, World!</h1>\n");
    }

    #[tokio::test]
    async fn test_get_template_by_name_with_context() {
        let dir = tempdir().unwrap();
        let template_path = dir.path().join("test_template.html");
        let mut file = File::create(&template_path).unwrap();
        writeln!(file, "<h1>Hello, {{ name }}!</h1>").unwrap();

        let config = SMTPConfig {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            smtp_username: "user".to_string(),
            smtp_password: "password".to_string(),
        };
        let mail_service = MailService::new(config, Some(dir.path().to_path_buf()))
            .await
            .unwrap();

        let mut context = Context::new();
        context.insert("name", "John Doe");
        let result = mail_service
            .build_template_by_name_with_context("test_template.html", context)
            .unwrap();

        let expected = "<h1>Hello, John Doe!</h1>";
        let result_trimmed = result.trim();

        assert_eq!(
            result_trimmed, expected,
            "Expected: '{}', but got: '{}'",
            expected, result_trimmed
        );
    }
}
