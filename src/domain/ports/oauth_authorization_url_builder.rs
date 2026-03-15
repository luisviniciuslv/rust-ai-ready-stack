use crate::domain::errors::DomainError;

pub trait OAuthAuthorizationUrlBuilder: Send + Sync {
    fn build_authorization_url(&self, redirect_uri: &str) -> Result<String, DomainError>;
}
