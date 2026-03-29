use crate::adapters::mongodb::mongo_repo::MongoRepo;
use crate::adapters::mongodb::to_domain_error;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::ports::UserRepository;
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, Regex};

#[async_trait]
impl UserRepository for MongoRepo {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError> {
        let user_col = self.get_user_collection();
        let obj_id = ObjectId::parse_str(id).map_err(|_| DomainError::InvalidId(id.to_string()))?;

        let user = user_col
            .find_one(doc! { "_id": obj_id })
            .await
            .map_err(to_domain_error)?;

        match user {
            Some(u) => u.into_domain().map(Some),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let user_col = self.get_user_collection();

        let user = user_col
            .find_one(doc! { "email": email })
            .await
            .map_err(to_domain_error)?;

        match user {
            Some(u) => u.into_domain().map(Some),
            None => Ok(None),
        }
    }

    async fn list_all(&self, email: Option<&str>) -> Result<Vec<User>, DomainError> {
        let user_col = self.get_user_collection();
        let mut filter = doc! {};

        if let Some(email_filter) = email.map(str::trim).filter(|v| !v.is_empty()) {
            filter.insert(
                "email",
                doc! {
                    "$regex": Regex {
                        pattern: email_filter.to_string(),
                        options: "i".to_string(),
                    }
                },
            );
        }
        let mut cursor = user_col.find(filter).await.map_err(to_domain_error)?;

        let mut users = Vec::new();
        while let Some(user_doc) = cursor.try_next().await.map_err(to_domain_error)? {
            users.push(user_doc.into_domain()?);
        }

        Ok(users)
    }

    async fn ensure_is_admin(&self, email: &str) -> Result<User, DomainError> {
        let user = self
            .find_by_email(email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Usuário não encontrado".to_string()))?;

        if !user.is_admin() {
            return Err(DomainError::Forbidden(
                "Acesso restrito a administradores".to_string(),
            ));
        }

        Ok(user)
    }

    async fn save_user(&self, user: User) -> Result<User, DomainError> {
    let user_col = self.get_user_collection();
    let filter = doc! { "email": user.email().as_str() };
    
    let update = doc! {
        "$set": {
            "email": user.email().as_str(),
            "is_admin": user.is_admin(),
        },
        "$setOnInsert": { "is_manager": false }
    };

    user_col.update_one(filter, update).await.map_err(to_domain_error)?;
    
    self.find_by_email(user.email().as_str()).await?
        .ok_or(DomainError::InternalError("Erro ao salvar usuário".to_string()))
}
}
