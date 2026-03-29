use crate::domain::entities::AuthIdentity;
use crate::domain::errors::DomainError;
use crate::domain::ports::{IdentityService, SessionCookieService};
use crate::domain::value_objects::Email;
use std::sync::Arc;

pub struct LocalSignInConfig {
    pub enabled: bool,
    pub email: String,
    pub password: String,
    pub name: String,
    pub picture: String,
}

pub struct LocalSignInResult {
    pub auth_cookie: String,
}

pub struct LocalSignInUseCase {
    identity_service: Arc<dyn IdentityService>,
    session_cookie_service: Arc<dyn SessionCookieService>,
    config: LocalSignInConfig,
}

impl LocalSignInUseCase {
    pub fn new(
        identity_service: Arc<dyn IdentityService>,
        session_cookie_service: Arc<dyn SessionCookieService>,
        config: LocalSignInConfig,
    ) -> Self {
        Self {
            identity_service,
            session_cookie_service,
            config,
        }
    }

    pub fn execute(&self, email: &str, password: &str) -> Result<LocalSignInResult, DomainError> {
        if !self.config.enabled {
            return Err(DomainError::Forbidden(
                "Autenticação local está desabilitada".to_string(),
            ));
        }

        let email_matches = email.trim().eq_ignore_ascii_case(&self.config.email);
        let password_matches = password == self.config.password;

        if !email_matches || !password_matches {
            return Err(DomainError::Unauthorized(
                "Credenciais inválidas".to_string(),
            ));
        }

        let email_value = Email::new(self.config.email.clone())?;
        let identity = AuthIdentity {
            sub: email_value.as_str().to_string(),
            email: email_value,
            name: self.config.name.clone(),
            picture: self.config.picture.clone(),
        };

        let token = self.identity_service.generate_token(&identity)?;
        let auth_cookie = self.session_cookie_service.build_auth_cookie(&token);

        Ok(LocalSignInResult { auth_cookie })
    }
}
