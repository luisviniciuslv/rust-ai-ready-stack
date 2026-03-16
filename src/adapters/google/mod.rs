use crate::domain::errors::DomainError;
use crate::domain::ports::{GoogleOAuthProvider, OAuthAuthorizationUrlBuilder};
use async_trait::async_trait;

pub mod google_oauth_authorization_url_builder;
pub mod google_oauth_provider;

pub use google_oauth_authorization_url_builder::GoogleOAuthAuthorizationUrlBuilder;
pub use google_oauth_provider::ReqwestGoogleOAuthProvider;

pub struct DisabledGoogleOAuthProvider;

#[async_trait]
impl GoogleOAuthProvider for DisabledGoogleOAuthProvider {
    async fn fetch_user_by_code(
        &self,
        _authorization_code: &str,
        _redirect_uri: &str,
    ) -> Result<crate::domain::entities::GoogleUser, DomainError> {
        Err(DomainError::Forbidden(
            "Google OAuth está desabilitado neste ambiente".to_string(),
        ))
    }
}

pub struct DisabledOAuthAuthorizationUrlBuilder;

impl OAuthAuthorizationUrlBuilder for DisabledOAuthAuthorizationUrlBuilder {
    fn build_authorization_url(&self, _redirect_uri: &str) -> Result<String, DomainError> {
        Err(DomainError::Forbidden(
            "Google OAuth está desabilitado neste ambiente".to_string(),
        ))
    }
}

pub(crate) fn url_parse_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::InvalidData("URL de autorização malformada".to_string())
}

pub(crate) fn oauth_http_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::BusinessRuleViolation("Falha na comunicação com provedor OAuth".to_string())
}

pub(crate) fn oauth_parse_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::InvalidData("Resposta OAuth inválida".to_string())
}
