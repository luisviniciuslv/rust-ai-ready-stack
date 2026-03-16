use crate::domain::entities::{AuthClaims, AuthIdentity};
use crate::domain::errors::DomainError;

pub trait IdentityService: Send + Sync {
    fn generate_token(&self, identity: &AuthIdentity) -> Result<String, DomainError>;
    fn validate_token(&self, token: &str) -> Result<AuthClaims, DomainError>;
}
