use crate::domain::entities::{Notification, User};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

fn default_notification_id() -> String {
    Uuid::new_v4().to_string()
}

mod optional_datetime {
    use chrono::{DateTime, Utc};
    use mongodb::bson::DateTime as BsonDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => {
                let bson_dt = BsonDateTime::from_millis(dt.timestamp_millis());
                serializer.serialize_some(&bson_dt)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<BsonDateTime> = Option::deserialize(deserializer)?;
        Ok(opt.and_then(|bson_dt| DateTime::from_timestamp_millis(bson_dt.timestamp_millis())))
    }
}

fn serialize_option_oid_as_hex<S>(oid: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match oid {
        Some(ref id) => serializer.serialize_str(&id.to_hex()),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDocument {
    #[serde(default = "default_notification_id")]
    pub id: String,
    pub case_id: String,
    pub message: String,
    pub concluded: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_datetime"
    )]
    pub created_at: Option<DateTime<Utc>>,
}

impl NotificationDocument {
    #[allow(dead_code)]
    pub fn from_domain(notification: Notification) -> Self {
        Self {
            id: notification.id().to_string(),
            case_id: notification.case_id().to_string(),
            message: notification.message().to_string(),
            concluded: notification.concluded(),
            created_at: notification.created_at(),
        }
    }

    pub fn into_domain(self) -> Notification {
        Notification::from_persisted(
            self.id,
            self.case_id,
            self.message,
            self.concluded,
            self.created_at,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDocument {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_oid_as_hex"
    )]
    pub id: Option<ObjectId>,
    pub email: String,
    pub department: ObjectId,
    pub is_manager: bool,
    pub is_admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<NotificationDocument>>,
}

impl UserDocument {
    pub fn into_domain(self) -> Result<User, crate::domain::errors::DomainError> {
        let email = crate::domain::value_objects::Email::new(self.email)?;
        let mut user = User::new(email, self.department.to_hex())
            .set_manager(self.is_manager)
            .set_admin(self.is_admin);

        if let Some(notifications) = self.notifications {
            user = user.with_notifications(
                notifications
                    .into_iter()
                    .map(NotificationDocument::into_domain)
                    .collect(),
            );
        }

        if let Some(id) = self.id {
            user = user.with_id(id.to_hex());
        }

        Ok(user)
    }
}
