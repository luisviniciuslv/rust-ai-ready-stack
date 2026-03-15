use crate::domain::errors::DomainError;

/// Porta de saída para envio de e-mail HTML.
#[allow(dead_code)]
pub trait EmailSender: Send + Sync {
    fn send_html_email(
        &self,
        recipient: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), DomainError>;
}
