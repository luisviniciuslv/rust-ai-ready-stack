use crate::domain::value_objects::Email;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Notification {
    id: String,
    case_id: String,
    message: String,
    concluded: bool,
    created_at: Option<DateTime<Utc>>,
}

impl Notification {
    #[allow(dead_code)]
    pub fn new(case_id: String, message: String, concluded: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            case_id,
            message,
            concluded,
            created_at: Some(Utc::now()),
        }
    }

    pub fn from_persisted(id: String, case_id: String, message: String, concluded: bool, created_at: Option<DateTime<Utc>>) -> Self {
        Self {
            id,
            case_id,
            message,
            concluded,
            created_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn case_id(&self) -> &str {
        &self.case_id
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn concluded(&self) -> bool {
        self.concluded
    }

    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }
}

/// Entidade de domínio User - Pura, sem dependências externas
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct User {
    id: Option<String>,
    email: Email,
    department_id: String,
    is_manager: bool,
    is_admin: bool,
    notifications: Vec<Notification>,
}

impl User {
    pub fn new(email: Email, department_id: String) -> Self {
        Self {
            id: None,
            email,
            department_id,
            is_manager: false,
            is_admin: false,
            notifications: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn set_manager(mut self, is_manager: bool) -> Self {
        self.is_manager = is_manager;
        self
    }

    pub fn set_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }

    pub fn with_notifications(mut self, notifications: Vec<Notification>) -> Self {
        self.notifications = notifications;
        self
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    #[allow(dead_code)]
    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn department_id(&self) -> &str {
        &self.department_id
    }

    pub fn is_manager(&self) -> bool {
        self.is_manager
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}
