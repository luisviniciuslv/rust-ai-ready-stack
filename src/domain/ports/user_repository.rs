use crate::domain::entities::{Notification, User};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Email;
use async_trait::async_trait;

/// Porta de saída para repositório de User (interface)
#[async_trait]
#[allow(dead_code)]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn list_all(
        &self,
        email: Option<&str>,
        department_id: Option<&str>,
    ) -> Result<Vec<User>, DomainError>;
    async fn ensure_is_admin(&self, email: &str) -> Result<User, DomainError>;
    async fn ensure_is_manager(&self, email: &str) -> Result<User, DomainError>;
    async fn save_with_department(
        &self,
        email: Email,
        department_id: &str,
    ) -> Result<(), DomainError>;
    async fn add_notification_to_users(
        &self,
        user_ids: &[String],
        notification: Notification,
    ) -> Result<(), DomainError>;
    async fn update_notification_concluded(
        &self,
        user_email: &str,
        notification_id: &str,
        concluded: bool,
    ) -> Result<(), DomainError>;
    async fn find_notifications_paginated(
        &self,
        user_email: &str,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Notification>, i64, i64), DomainError>;
    async fn conclude_all_notifications(&self, user_email: &str) -> Result<(), DomainError>;
}
