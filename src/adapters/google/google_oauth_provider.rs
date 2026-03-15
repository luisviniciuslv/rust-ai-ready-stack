use crate::adapters::google::{oauth_http_error, oauth_parse_error};
use crate::domain::entities::GoogleUser;
use crate::domain::errors::DomainError;
use crate::domain::ports::GoogleOAuthProvider;
use crate::domain::value_objects::Email;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct ReqwestGoogleOAuthProvider {
    client: Client,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GoogleUserResponse {
    sub: String,
    email: String,
    email_verified: bool,
    name: String,
    picture: String,
}

impl GoogleUserResponse {
    fn into_domain(self) -> Result<GoogleUser, DomainError> {
        let email = Email::new(self.email)?;
        Ok(GoogleUser {
            sub: self.sub,
            email,
            email_verified: self.email_verified,
            name: self.name,
            picture: self.picture,
        })
    }
}

impl ReqwestGoogleOAuthProvider {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
        }
    }
}

#[async_trait]
impl GoogleOAuthProvider for ReqwestGoogleOAuthProvider {
    async fn fetch_user_by_code(
        &self,
        authorization_code: &str,
        redirect_uri: &str,
    ) -> Result<GoogleUser, DomainError> {
        let token_response = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", authorization_code),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("redirect_uri", redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(oauth_http_error)?
            .error_for_status()
            .map_err(oauth_http_error)?
            .json::<TokenResponse>()
            .await
            .map_err(oauth_parse_error)?;

        let user_info = self
            .client
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(&token_response.access_token)
            .send()
            .await
            .map_err(oauth_http_error)?
            .error_for_status()
            .map_err(oauth_http_error)?
            .json::<GoogleUserResponse>()
            .await
            .map_err(oauth_parse_error)?;

        user_info.into_domain()
    }
}
