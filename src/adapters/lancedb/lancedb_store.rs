use anyhow::{Context, Result};
use arrow_array::{
    builder::{FixedSizeListBuilder, PrimitiveBuilder},
    types::Float32Type,
    RecordBatch, RecordBatchIterator, StringArray,
};
use futures::TryStreamExt;
use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::{connect, Table};
use std::sync::Arc;

use crate::domain::entities::{DocumentChunk, ProcessCategory};

#[derive(Clone)]
pub struct LanceDbRepo {
    table: Table,
}

const DIM: i32 = 1536;

impl LanceDbRepo {
    pub async fn new(uri: &str, table_name: &str) -> Result<Self> {
        let conn = connect(uri).execute().await?;

        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("content", DataType::Utf8, false),
            Field::new("category", DataType::Utf8, false),
            Field::new("source", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), DIM),
                false,
            ),
        ]));

        let table = if conn
            .table_names()
            .execute()
            .await?
            .contains(&table_name.to_string())
        {
            conn.open_table(table_name).execute().await?
        } else {
            conn.create_empty_table(table_name, schema)
                .execute()
                .await?
        };

        Ok(Self { table })
    }

    pub async fn add_documents(
        &self,
        chunks: Vec<DocumentChunk>,
        embeddings: Vec<Vec<f32>>,
    ) -> Result<()> {
        if chunks.is_empty() || chunks.len() != embeddings.len() {
            return Ok(());
        }

        let len = chunks.len();

        let ids: Vec<String> = (0..len).map(|_| uuid::Uuid::new_v4().to_string()).collect();
        let contents: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let categories: Vec<String> = chunks.iter().map(|c| c.category.to_string()).collect();
        let sources: Vec<String> = chunks.iter().map(|c| c.source_filename.clone()).collect();

        let mut list_builder =
            FixedSizeListBuilder::new(PrimitiveBuilder::<Float32Type>::new(), DIM);

        for vec in embeddings {
            if vec.len() as i32 != DIM {
                return Err(anyhow::anyhow!(
                    "Dimensão incorreta. Modelo gera 1536, esperado {DIM}"
                ));
            }
            for val in vec {
                list_builder.values().append_value(val);
            }
            list_builder.append(true);
        }
        let vector_array = list_builder.finish();

        let schema = self.table.schema().await?;
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(ids)),
                Arc::new(StringArray::from(contents)),
                Arc::new(StringArray::from(categories)),
                Arc::new(StringArray::from(sources)),
                Arc::new(vector_array),
            ],
        )?;

        let batches = RecordBatchIterator::new(vec![Ok(batch)].into_iter(), schema);

        self.table.add(batches).execute().await?;

        Ok(())
    }

    pub async fn search(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
        category: Option<String>,
    ) -> Result<Vec<DocumentChunk>> {
        let mut query_builder = self.table.query().nearest_to(query_vector)?;

        if let Some(cat) = category {
            query_builder = query_builder.only_if(format!("category = '{}'", cat));
        }

        let results = query_builder
            .limit(limit)
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        let mut final_docs = Vec::new();

        for batch in results {
            let content_col = batch
                .column_by_name("content")
                .context("Coluna 'content' não encontrada")?
                .as_any()
                .downcast_ref::<StringArray>()
                .context("Erro no downcast da coluna 'content'")?;

            let category_col = batch
                .column_by_name("category")
                .context("Coluna 'category' não encontrada")?
                .as_any()
                .downcast_ref::<StringArray>()
                .context("Erro no downcast da coluna 'category'")?;

            let source_col = batch
                .column_by_name("source")
                .context("Coluna 'source' não encontrada")?
                .as_any()
                .downcast_ref::<StringArray>()
                .context("Erro no downcast da coluna 'source'")?;

            for i in 0..batch.num_rows() {
                let content = content_col.value(i).to_string();
                let category_str = category_col.value(i);
                let source = source_col.value(i).to_string();

                let category = category_str.parse().unwrap_or(ProcessCategory::Commercial);

                final_docs.push(DocumentChunk {
                    content,
                    category,
                    source_filename: source,
                });
            }
        }

        Ok(final_docs)
    }
}
