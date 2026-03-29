use crate::domain::entities::{AuthIdentity, GoogleUser, User};
use crate::domain::errors::DomainError;
use crate::domain::ports::{GoogleOAuthProvider, IdentityService, SessionCookieService, UserRepository};
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
    require_verified_email: bool,
    allowed_email_domains: Vec<String>,
    user_repository: Arc<dyn UserRepository>,
}

impl AuthorizeGoogleUserUseCase {
    pub fn new(
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        identity_service: Arc<dyn IdentityService>,
        session_cookie_service: Arc<dyn SessionCookieService>,
        require_verified_email: bool,
        allowed_email_domains: Vec<String>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            google_oauth_provider,
            identity_service,
            session_cookie_service,
            require_verified_email,
            allowed_email_domains,
            user_repository,
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


        let identity = self.ensure_allowed_user(user).ok_or(DomainError::Unauthorized(
            "Usuário não autorizado".to_string()
        ))?;

        let domain_user = User::new(identity.email.clone()); 

        let _ = self.user_repository.save_user(domain_user).await?;

        let token = self.identity_service.generate_token(&identity)?;
        let auth_cookie = self.session_cookie_service.build_auth_cookie(&token);

        Ok(AuthorizeGoogleUserResult {
            redirect_url: frontend_url.to_string(),
            auth_cookie: Some(auth_cookie),
        })
    }

    fn ensure_allowed_user(&self, user: GoogleUser) -> Option<AuthIdentity> {
        if self.require_verified_email && !user.email_verified {
            return None;
        }

        let email_domain = user
            .email
            .as_str()
            .split('@')
            .next_back()
            .map(|d| d.to_lowercase());

        let is_domain_allowed = self.allowed_email_domains.is_empty()
            || email_domain
                .as_ref()
                .is_some_and(|domain| self.allowed_email_domains.iter().any(|d| d == domain));

        if !is_domain_allowed {
            return None;
        }

        Some(AuthIdentity {
            sub: user.sub,
            email: user.email,
            name: user.name,
            picture: user.picture,
        })
    }
}
