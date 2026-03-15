use crate::domain::entities::{AuthClaims, GoogleUser};
use crate::domain::errors::DomainError;

pub trait IdentityService: Send + Sync {
    fn generate_token(&self, user: &GoogleUser) -> Result<String, DomainError>;
    fn validate_token(&self, token: &str) -> Result<AuthClaims, DomainError>;
}
