use crate::domain::errors::DomainError;

pub mod google_oauth_authorization_url_builder;
pub mod google_oauth_provider;

pub use google_oauth_authorization_url_builder::GoogleOAuthAuthorizationUrlBuilder;
pub use google_oauth_provider::ReqwestGoogleOAuthProvider;

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
