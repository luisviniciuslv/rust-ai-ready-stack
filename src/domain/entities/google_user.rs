use crate::domain::value_objects::Email;

#[derive(Debug, Clone)]
pub struct GoogleUser {
    pub sub: String,
    pub email: Email,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
}
