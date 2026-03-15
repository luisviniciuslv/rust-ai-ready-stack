#![allow(dead_code)]

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

#[derive(Clone)]
pub struct AuthConfig {
    pub email_address: String,
    pub password: String,
}

pub fn enviar_email(
    destinatario: &str,
    autenticacao: &AuthConfig,
    titulo: &str,
    mensagem_html: &str,
) -> Result<(), Box<dyn Error>> {
    // 1. Montagem do E-mail
    // .singlepart() aqui cria um corpo HTML direto, similar ao seu add_alternative
    let email = Message::builder()
        .from(autenticacao.email_address.parse()?)
        .to(destinatario.parse()?)
        .subject(titulo)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(String::from(mensagem_html))?;

    // 2. Credenciais
    let creds = Credentials::new(
        autenticacao.email_address.clone(),
        autenticacao.password.clone(),
    );

    // 3. Transporte (Gmail porta 465 exige TLS implícito)
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // 4. Envio
    mailer.send(&email)?;

    Ok(())
}