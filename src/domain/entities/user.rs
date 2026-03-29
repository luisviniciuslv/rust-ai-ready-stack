use crate::domain::value_objects::Email;

/// Entidade de domínio User - Pura, sem dependências externas
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct User {
    id: Option<String>,
    email: Email,
    is_admin: bool,
}

impl User {
    pub fn new(email: Email) -> Self {
        Self {
            id: None,
            email,
            is_admin: false,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn set_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    #[allow(dead_code)]
    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}
