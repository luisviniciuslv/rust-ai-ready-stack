use crate::domain::errors::DomainError;
use crate::domain::ports::OAuthAuthorizationUrlBuilder;
use std::sync::Arc;

pub struct BuildGoogleOAuthUrlUseCase {
    oauth_authorization_url_builder: Arc<dyn OAuthAuthorizationUrlBuilder>,
}

impl BuildGoogleOAuthUrlUseCase {
    pub fn new(oauth_authorization_url_builder: Arc<dyn OAuthAuthorizationUrlBuilder>) -> Self {
        Self {
            oauth_authorization_url_builder,
        }
    }

    pub fn execute(&self, backend_url: &str) -> Result<String, DomainError> {
        let redirect_uri = format!("{}/auth/callback", backend_url);
        self.oauth_authorization_url_builder
            .build_authorization_url(&redirect_uri)
    }
}
