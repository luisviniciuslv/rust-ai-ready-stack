use crate::domain::entities::GoogleUser;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait GoogleOAuthProvider: Send + Sync {
    async fn fetch_user_by_code(
        &self,
        authorization_code: &str,
        redirect_uri: &str,
    ) -> Result<GoogleUser, DomainError>;
}
