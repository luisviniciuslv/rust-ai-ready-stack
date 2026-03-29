use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Porta de saída para repositório de User (interface)
#[async_trait]
#[allow(dead_code)]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn list_all(&self, email: Option<&str>) -> Result<Vec<User>, DomainError>;
    async fn ensure_is_admin(&self, email: &str) -> Result<User, DomainError>;
    async fn save_user(&self, user: User) -> Result<User, DomainError>;
}
