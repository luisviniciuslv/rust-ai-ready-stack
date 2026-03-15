use crate::{
    error::AppResult,
    state::AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct IngestRequest {
    pub file_path: String,
    pub category: String,
}

#[derive(Serialize)]
pub struct IngestResponse {
    pub message: String,
    pub total_chunks: usize,
}

pub async fn ingest_document(
    State(state): State<Arc<AppState>>,
    Json(req): Json<IngestRequest>,
) -> AppResult<impl IntoResponse> {
    let result = state
        .use_cases
        .ingest_document
        .execute(&req.file_path, &req.category)
        .await?;

    Ok(Json(IngestResponse {
        message: result.message,
        total_chunks: result.total_chunks,
    }))
}
