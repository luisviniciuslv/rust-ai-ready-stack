use crate::adapters::jwt::{jwt_decode_error, jwt_encode_error};
use crate::domain::entities::{AuthClaims, AuthIdentity};
use crate::domain::errors::DomainError;
use crate::domain::ports::IdentityService;
use crate::domain::value_objects::Email;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JwtClaimsDto {
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
    email: String,
    name: String,
    picture: String,
}

impl JwtClaimsDto {
    fn into_auth_claims(self) -> Result<AuthClaims, DomainError> {
        let email = Email::new(self.email)?;
        Ok(AuthClaims {
            email: email.to_string(),
            name: self.name,
            picture: self.picture,
        })
    }
}

pub struct JwtIdentityService {
    secret: String,
    audience: String,
    issuer: String,
    expires_in_hours: i64,
    leeway_seconds: u64,
}

impl JwtIdentityService {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            audience: "company-compass".to_string(),
            issuer: "company-compass".to_string(),
            expires_in_hours: 24,
            leeway_seconds: 60,
        }
    }
}

impl IdentityService for JwtIdentityService {
    fn generate_token(&self, identity: &AuthIdentity) -> Result<String, DomainError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::hours(self.expires_in_hours)).timestamp() as usize;

        let claims = JwtClaimsDto {
            aud: self.audience.clone(),
            exp,
            iat,
            iss: self.issuer.clone(),
            sub: identity.sub.clone(),
            email: identity.email.as_str().to_string(),
            name: identity.name.clone(),
            picture: identity.picture.clone(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(jwt_encode_error)
    }

    fn validate_token(&self, token: &str) -> Result<AuthClaims, DomainError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[self.audience.as_str()]);
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.leeway = self.leeway_seconds;

        let token_data = decode::<JwtClaimsDto>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )
        .map_err(jwt_decode_error)?;

        token_data.claims.into_auth_claims()
    }
}
