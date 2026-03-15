use crate::domain::errors::DomainError;
use crate::domain::ports::EmailSender;
use crate::utils::send_email::{enviar_email, AuthConfig};

#[allow(dead_code)]
pub struct SmtpEmailSender {
    auth: AuthConfig,
}

#[allow(dead_code)]
impl SmtpEmailSender {
    pub fn new(auth: AuthConfig) -> Self {
        Self { auth }
    }
}

impl EmailSender for SmtpEmailSender {
    fn send_html_email(
        &self,
        recipient: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), DomainError> {
        enviar_email(recipient, &self.auth, subject, html_body).map_err(|_| {
            DomainError::BusinessRuleViolation(
                "Falha no envio de e-mail de pendência".to_string(),
            )
        })?;

        Ok(())
    }
}
