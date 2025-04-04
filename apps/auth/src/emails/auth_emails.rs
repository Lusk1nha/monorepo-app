use lettre::message::header::ContentType;
use mail_service::EmailRequest;
use tera::Context;

pub enum AuthEmailType {
    OtpCode {
        from: String,
        to: String,
        code: String,
    },
    ConfirmEmail {
        from: String,
        to: String,
        confirmation_link: String,
    },
}

impl AuthEmailType {
    pub fn get_template_name(&self) -> &str {
        match self {
            Self::OtpCode { .. } => "otp_code.html",
            Self::ConfirmEmail { .. } => "confirm_email.html",
        }
    }

    pub fn build_request(self) -> EmailRequest {
        match self {
            Self::OtpCode {
                ref from, ref to, ..
            } => {
                let context = self.build_context();

                EmailRequest {
                    from: from.clone(),
                    to: to.clone(),
                    header: Some(ContentType::TEXT_HTML),
                    subject: "Authentication OTP Code".to_string(),
                    template_name: self.get_template_name().to_string(),
                    context: Some(context),
                }
            }

            Self::ConfirmEmail {
                ref from, ref to, ..
            } => {
                let context: Context = self.build_context();

                EmailRequest {
                    from: from.clone(),
                    to: to.clone(),
                    header: Some(ContentType::TEXT_HTML),
                    subject: "Confirm Email".to_string(),
                    template_name: self.get_template_name().to_string(),
                    context: Some(context),
                }
            }
        }
    }

    pub fn build_context(&self) -> Context {
        match self {
            Self::OtpCode { code, .. } => {
                let mut context = Context::new();
                context.insert("code", code);
                context
            }

            Self::ConfirmEmail {
                confirmation_link, ..
            } => {
                let mut context = Context::new();
                context.insert("confirmation_link", confirmation_link);
                context
            }
        }
    }
}
