use crate::adapters::google::url_parse_error;
use crate::domain::errors::DomainError;
use crate::domain::ports::OAuthAuthorizationUrlBuilder;
use reqwest::Url;

pub struct GoogleOAuthAuthorizationUrlBuilder {
    client_id: String,
}

impl GoogleOAuthAuthorizationUrlBuilder {
    pub fn new(client_id: String) -> Self {
        Self { client_id }
    }
}

impl OAuthAuthorizationUrlBuilder for GoogleOAuthAuthorizationUrlBuilder {
    fn build_authorization_url(&self, redirect_uri: &str) -> Result<String, DomainError> {
        let mut url =
            Url::parse("https://accounts.google.com/o/oauth2/v2/auth").map_err(url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", "openid email profile")
            .append_pair("access_type", "offline");

        Ok(url.into())
    }
}
