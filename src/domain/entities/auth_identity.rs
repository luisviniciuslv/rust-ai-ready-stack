use crate::domain::value_objects::Email;

#[derive(Debug, Clone)]
pub struct AuthIdentity {
    pub sub: String,
    pub email: Email,
    pub name: String,
    pub picture: String,
}
