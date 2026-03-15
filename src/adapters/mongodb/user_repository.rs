use crate::adapters::mongodb::models::{NotificationDocument, UserDocument};
use crate::adapters::mongodb::mongo_repo::MongoRepo;
use crate::adapters::mongodb::to_domain_error;
use crate::domain::entities::{Notification, User};
use crate::domain::errors::DomainError;
use crate::domain::ports::UserRepository;
use crate::domain::value_objects::Email;
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, Bson, Regex};
use mongodb::Collection;
use mongodb::bson::Document;

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

    async fn list_all(
        &self,
        email: Option<&str>,
        department_id: Option<&str>,
    ) -> Result<Vec<User>, DomainError> {
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

        if let Some(department_filter) = department_id.map(str::trim).filter(|v| !v.is_empty()) {
            let department_object_id = ObjectId::parse_str(department_filter)
                .map_err(|_| DomainError::InvalidId(department_filter.to_string()))?;
            filter.insert("department", department_object_id);
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

    async fn ensure_is_manager(&self, email: &str) -> Result<User, DomainError> {
        let user = self
            .find_by_email(email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Usuário não encontrado".to_string()))?;

        if !user.is_manager() {
            return Err(DomainError::Forbidden(
                "Acesso restrito a gerentes".to_string(),
            ));
        }

        Ok(user)
    }

    async fn save_with_department(
        &self,
        email: Email,
        department_id: &str,
    ) -> Result<(), DomainError> {
        let user_col = self.get_user_collection();
        let dept_id = ObjectId::parse_str(department_id)
            .map_err(|_| DomainError::InvalidId(department_id.to_string()))?;

        let email_str = email.into_string();

        // Verificar se o usuário já existe
        if user_col
            .find_one(doc! { "email": &email_str })
            .await
            .map_err(to_domain_error)?
            .is_some()
        {
            return Err(DomainError::Conflict(format!(
                "Usuário com email {} já existe",
                email_str
            )));
        }

        let new_user = UserDocument {
            id: None,
            email: email_str,
            department: dept_id,
            is_manager: false,
            is_admin: false,
            notifications: None,
        };

        user_col
            .insert_one(new_user)
            .await
            .map_err(to_domain_error)?;

        Ok(())
    }

    async fn add_notification_to_users(
        &self,
        user_ids: &[String],
        notification: Notification,
    ) -> Result<(), DomainError> {
        let user_col = self.get_user_collection();

        if user_ids.is_empty() {
            return Ok(());
        }

        let mut object_ids = Vec::with_capacity(user_ids.len());
        for user_id in user_ids {
            object_ids.push(
                ObjectId::parse_str(user_id)
                    .map_err(|_| DomainError::InvalidId(user_id.to_string()))?,
            );
        }

        let notification_doc = NotificationDocument::from_domain(notification);
        let notification_bson = mongodb::bson::to_document(&notification_doc)
            .map_err(|e| DomainError::InvalidData(format!("Erro ao serializar notificação: {}", e)))?;

        let filter = doc! { "_id": { "$in": object_ids } };
        let update = doc! {
            "$push": {
                "notifications": notification_bson
            }
        };

        user_col
            .update_many(filter, update)
            .await
            .map_err(to_domain_error)?;

        Ok(())
    }

    async fn update_notification_concluded(
        &self,
        user_email: &str,
        notification_id: &str,
        concluded: bool,
    ) -> Result<(), DomainError> {
        let user_col = self.get_user_collection();

        let filter = doc! {
            "email": user_email,
            "notifications.id": notification_id,
        };

        let update = doc! {
            "$set": {
                "notifications.$.concluded": concluded
            }
        };

        let result = user_col
            .update_one(filter, update)
            .await
            .map_err(to_domain_error)?;

        if result.matched_count == 0 {
            return Err(DomainError::NotFound(format!(
                "Notificação não encontrada para o usuário autenticado: {}",
                notification_id
            )));
        }

        Ok(())
    }

    async fn find_notifications_paginated(
        &self,
        user_email: &str,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Notification>, i64, i64), DomainError> {
        let user_col: Collection<Document> = self.get_user_collection_as_document();
        let skip = (page - 1) * page_size;
        let limit = page_size;

        let pipeline = vec![
            doc! {
                "$match": { "email": user_email }
            },
            doc! {
                "$facet": {
                    "metadata": [
                        {
                            "$project": {
                                "notifications": { "$ifNull": ["$notifications", []] }
                            }
                        },
                        {
                            "$unwind": {
                                "path": "$notifications",
                                "preserveNullAndEmptyArrays": false
                            }
                        },
                        {
                            "$group": {
                                "_id": Bson::Null,
                                "total": { "$sum": 1 },
                                "total_unconcluded": {
                                    "$sum": {
                                        "$cond": [
                                            { "$eq": ["$notifications.concluded", false] },
                                            1,
                                            0
                                        ]
                                    }
                                }
                            }
                        }
                    ],
                    "data": [
                        {
                            "$project": {
                                "notifications": { "$ifNull": ["$notifications", []] }
                            }
                        },
                        {
                            "$unwind": {
                                "path": "$notifications",
                                "preserveNullAndEmptyArrays": false
                            }
                        },
                        {
                            "$sort": {
                                "notifications.concluded": 1,
                                "notifications.created_at": 1
                            }
                        },
                        { "$skip": skip },
                        { "$limit": limit },
                        {
                            "$project": {
                                "id": "$notifications.id",
                                "case_id": "$notifications.case_id",
                                "message": "$notifications.message",
                                "concluded": "$notifications.concluded",
                                "created_at": "$notifications.created_at"
                            }
                        }
                    ]
                }
            }
        ];

        let mut cursor = user_col
            .aggregate(pipeline)
            .await
            .map_err(to_domain_error)?;

        if let Some(result_doc) = cursor
            .try_next()
            .await
            .map_err(to_domain_error)?
        {
            let mut notifications = Vec::new();
            let mut total_items: i64 = 0;
            let mut total_unconcluded_items: i64 = 0;

            // Extract total count from metadata
            if let Ok(metadata_array) = result_doc.get_array("metadata") {
                if let Some(metadata_doc) = metadata_array.first().and_then(|v| v.as_document()) {
                    total_items = metadata_doc
                        .get("total")
                        .and_then(|v| v.as_i64().or_else(|| v.as_i32().map(|i| i as i64)))
                        .unwrap_or(0);

                    total_unconcluded_items = metadata_doc
                        .get("total_unconcluded")
                        .and_then(|v| v.as_i64().or_else(|| v.as_i32().map(|i| i as i64)))
                        .unwrap_or(0);
                }
            }

            // Extract notification data
            if let Ok(data_array) = result_doc.get_array("data") {
                for bson_val in data_array {
                    if let Ok(notification_doc) = mongodb::bson::from_bson::<NotificationDocument>(bson_val.clone()) {
                        notifications.push(notification_doc.into_domain());
                    }
                }
            }

            return Ok((notifications, total_items, total_unconcluded_items));
        }

        Ok((Vec::new(), 0, 0))
    }

    async fn conclude_all_notifications(&self, user_email: &str) -> Result<(), DomainError> {
        let user_col = self.get_user_collection();

        let filter = doc! { "email": user_email };
        let update = doc! {
            "$set": { "notifications.$[elem].concluded": true }
        };

        user_col
            .update_one(filter, update)
            .array_filters(vec![doc! { "elem.concluded": false }])
            .await
            .map_err(to_domain_error)?;

        Ok(())
    }
}
