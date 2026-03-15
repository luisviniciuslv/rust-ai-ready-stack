use crate::domain::entities::GoogleUser;
use crate::domain::errors::DomainError;
use crate::domain::ports::{GoogleOAuthProvider, IdentityService, SessionCookieService};
use std::sync::Arc;

pub struct AuthorizeGoogleUserResult {
    pub redirect_url: String,
    pub auth_cookie: Option<String>,
}

/// Caso de uso: autorizar usuário autenticado via Google OAuth
pub struct AuthorizeGoogleUserUseCase {
    google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
    identity_service: Arc<dyn IdentityService>,
    session_cookie_service: Arc<dyn SessionCookieService>,
}

impl AuthorizeGoogleUserUseCase {
    pub fn new(
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        identity_service: Arc<dyn IdentityService>,
        session_cookie_service: Arc<dyn SessionCookieService>,
    ) -> Self {
        Self {
            google_oauth_provider,
            identity_service,
            session_cookie_service,
        }
    }

    pub async fn execute(
        &self,
        authorization_code: &str,
        backend_url: &str,
        frontend_url: &str,
    ) -> Result<AuthorizeGoogleUserResult, DomainError> {
        let redirect_uri = format!("{}/auth/callback", backend_url);

        let user = self
            .google_oauth_provider
            .fetch_user_by_code(authorization_code, &redirect_uri)
            .await?;

        let Some(user) = Self::ensure_allowed_user(user) else {
            return Ok(AuthorizeGoogleUserResult {
                redirect_url: frontend_url.to_string(),
                auth_cookie: None,
            });
        };

        let token = self.identity_service.generate_token(&user)?;
        let auth_cookie = self.session_cookie_service.build_auth_cookie(&token);

        Ok(AuthorizeGoogleUserResult {
            redirect_url: frontend_url.to_string(),
            auth_cookie: Some(auth_cookie),
        })
    }

    fn ensure_allowed_user(user: GoogleUser) -> Option<GoogleUser> {
        if !user.email_verified {
            return None;
        }

        let is_domain_allowed = matches!(
            user.email.as_str().split('@').next_back(),
            Some("tudoemnuvem.com.br") | Some("tudoemnuvem.tec.br")
        );

        if !is_domain_allowed {
            return None;
        }

        Some(user)
    }
}
