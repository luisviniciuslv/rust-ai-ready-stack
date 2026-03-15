use crate::domain::errors::DomainError;
use crate::domain::ports::UserRepository;
use std::sync::Arc;

pub struct ProfileData {
    pub user_id: String,
    pub department_id: String,
    pub is_manager: bool,
    pub is_admin: bool,
}

/// Caso de uso: Obter dados de perfil do usuário autenticado
pub struct GetUserProfileUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUserProfileUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, email: &str) -> Result<ProfileData, DomainError> {
        let user = self
            .user_repository
            .find_by_email(email)
            .await?
            .ok_or_else(|| DomainError::NotFound("Usuário não encontrado".to_string()))?;

        let user_id = user
            .id()
            .map(|id| id.to_string())
            .ok_or_else(|| DomainError::InvalidData("Usuário autenticado sem id".to_string()))?;

        Ok(ProfileData {
            user_id,
            department_id: user.department_id().to_string(),
            is_manager: user.is_manager(),
            is_admin: user.is_admin(),
        })
    }
}
