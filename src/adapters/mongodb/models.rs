use crate::domain::entities::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

fn serialize_option_oid_as_hex<S>(oid: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match oid {
        Some(ref id) => serializer.serialize_str(&id.to_hex()),
        None => serializer.serialize_none(),
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
    pub is_manager: bool,
    pub is_admin: bool,
}

impl UserDocument {
    pub fn into_domain(self) -> Result<User, crate::domain::errors::DomainError> {
        let email = crate::domain::value_objects::Email::new(self.email)?;
        let mut user = User::new(email).set_admin(self.is_admin);

        if let Some(id) = self.id {
            user = user.with_id(id.to_hex());
        }

        Ok(user)
    }
}
