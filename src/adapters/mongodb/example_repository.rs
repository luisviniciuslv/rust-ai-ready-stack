use crate::adapters::mongodb::mongo_repo::MongoRepo;
use crate::adapters::mongodb::to_domain_error;
use crate::domain::errors::DomainError;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document};

#[derive(Clone)]
#[allow(dead_code)]
pub struct ExampleRepository {
    repo: MongoRepo,
}

#[allow(dead_code)]
impl ExampleRepository {
    pub fn new(repo: MongoRepo) -> Self {
        Self { repo }
    }

    pub async fn create(&self, key: &str, payload: Document) -> Result<String, DomainError> {
        let collection = self.repo.get_example_collection_as_document();

        let document = doc! {
            "key": key,
            "payload": payload,
            "created_at": BsonDateTime::now(),
            "updated_at": BsonDateTime::now(),
        };

        let insert_result = collection
            .insert_one(document)
            .await
            .map_err(to_domain_error)?;

        let id = insert_result
            .inserted_id
            .as_object_id()
            .map(ObjectId::to_hex)
            .ok_or_else(|| {
                DomainError::InvalidData("Falha ao obter id do registro criado".to_string())
            })?;

        Ok(id)
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<Document>, DomainError> {
        self.repo
            .get_example_collection_as_document()
            .find_one(doc! { "key": key })
            .await
            .map_err(to_domain_error)
    }

    pub async fn list(&self, page: u64, page_size: u64) -> Result<Vec<Document>, DomainError> {
        let page = page.max(1);
        let page_size = page_size.max(1).min(100);
        let skip = (page - 1) * page_size;

        let mut cursor = self
            .repo
            .get_example_collection_as_document()
            .find(doc! {})
            .sort(doc! { "created_at": -1 })
            .skip(skip)
            .limit(page_size as i64)
            .await
            .map_err(to_domain_error)?;

        let mut items = Vec::new();
        while cursor.advance().await.map_err(to_domain_error)? {
            items.push(cursor.deserialize_current().map_err(to_domain_error)?);
        }

        Ok(items)
    }

    pub async fn update_payload(&self, key: &str, payload: Document) -> Result<(), DomainError> {
        let update_result = self
            .repo
            .get_example_collection_as_document()
            .update_one(
                doc! { "key": key },
                doc! {
                    "$set": {
                        "payload": payload,
                        "updated_at": BsonDateTime::now(),
                    }
                },
            )
            .await
            .map_err(to_domain_error)?;

        if update_result.matched_count == 0 {
            return Err(DomainError::NotFound(format!(
                "Registro de exemplo não encontrado para key: {}",
                key
            )));
        }

        Ok(())
    }

    pub async fn delete_by_key(&self, key: &str) -> Result<(), DomainError> {
        self.repo
            .get_example_collection_as_document()
            .delete_one(doc! { "key": key })
            .await
            .map_err(to_domain_error)?;

        Ok(())
    }
}
