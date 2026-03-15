use crate::domain::errors::DomainError;

pub mod jwt_identity_service;

pub use jwt_identity_service::JwtIdentityService;

pub(crate) fn jwt_encode_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::Unauthorized("Falha ao gerar token de autenticação".to_string())
}

pub(crate) fn jwt_decode_error<E>(_: E) -> DomainError
where
    E: std::fmt::Display,
{
    DomainError::Unauthorized("Token de autenticação inválido ou expirado".to_string())
}
