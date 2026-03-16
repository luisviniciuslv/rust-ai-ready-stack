use crate::adapters::mongodb::models::UserDocument;
use crate::adapters::mongodb::to_app_error;
use crate::error::AppResult;
use mongodb::{
    bson::Document,
    bson::doc,
    options::{ClientOptions, IndexOptions, Tls, TlsOptions},
    Client, Collection, Database, IndexModel,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Clone)]
pub struct MongoRepo {
    pub db: Database,
}

impl MongoRepo {
    pub async fn new(uri: &str, db_name: &str) -> AppResult<Self> {
        // Parse da string de conexão base (host, porta, credenciais)
        let mut client_options = ClientOptions::parse(uri).await.map_err(to_app_error)?;

        let tls_enabled = env::var("MONGODB_TLS_ENABLED")
            .ok()
            .map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            })
            .unwrap_or(false);

        if tls_enabled {
            let allow_invalid_certificates = env::var("MONGODB_TLS_ALLOW_INVALID_CERTIFICATES")
                .ok()
                .map(|value| {
                    matches!(
                        value.trim().to_ascii_lowercase().as_str(),
                        "1" | "true" | "yes" | "on"
                    )
                })
                .unwrap_or(true);

            let ca_file_path = env::var("MONGODB_TLS_CA_FILE")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty());

            let cert_key_file_path = env::var("MONGODB_TLS_CERT_KEY_FILE")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty());

            let tls_config = match (ca_file_path, cert_key_file_path) {
                (Some(ca_path), Some(cert_path)) => TlsOptions::builder()
                    .allow_invalid_certificates(allow_invalid_certificates)
                    .ca_file_path(PathBuf::from(ca_path))
                    .cert_key_file_path(PathBuf::from(cert_path))
                    .build(),
                (Some(ca_path), None) => TlsOptions::builder()
                    .allow_invalid_certificates(allow_invalid_certificates)
                    .ca_file_path(PathBuf::from(ca_path))
                    .build(),
                (None, Some(cert_path)) => TlsOptions::builder()
                    .allow_invalid_certificates(allow_invalid_certificates)
                    .cert_key_file_path(PathBuf::from(cert_path))
                    .build(),
                (None, None) => TlsOptions::builder()
                    .allow_invalid_certificates(allow_invalid_certificates)
                    .build(),
            };
            client_options.tls = Some(Tls::Enabled(tls_config));
        }

        // Cria o cliente com as opções completas
        let client = Client::with_options(client_options).map_err(to_app_error)?;

        let db = client.database(db_name);

        Ok(Self { db })
    }
    fn _get_collection<T>(&self, name: &str) -> Collection<T>
    where
        T: Send + Sync + for<'de> Deserialize<'de> + Serialize,
    {
        self.db.collection::<T>(name)
    }
}

impl MongoRepo {
    pub fn get_user_collection(&self) -> Collection<UserDocument> {
        self._get_collection::<UserDocument>("users")
    }

    #[allow(dead_code)]
    pub fn get_user_collection_as_document(&self) -> Collection<Document> {
        self.db.collection::<Document>("users")
    }

    pub fn get_collection_as_document(&self, collection_name: &str) -> Collection<Document> {
        self.db.collection::<Document>(collection_name)
    }

    pub fn get_example_collection_as_document(&self) -> Collection<Document> {
        self.get_collection_as_document("examples")
    }

    pub async fn ensure_indexes(&self) -> AppResult<()> {
        let users_col = self.get_user_collection();
        let examples_col = self.get_example_collection_as_document();

        let users_email_index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(
                IndexOptions::builder()
                    .name("users_email_idx".to_string())
                    .unique(true)
                    .build(),
            )
            .build();

        users_col
            .create_index(users_email_index)
            .await
            .map_err(to_app_error)?;

        let examples_key_index = IndexModel::builder()
            .keys(doc! { "key": 1 })
            .options(
                IndexOptions::builder()
                    .name("examples_key_idx".to_string())
                    .unique(true)
                    .build(),
            )
            .build();

        examples_col
            .create_index(examples_key_index)
            .await
            .map_err(to_app_error)?;

        Ok(())
    }
}
