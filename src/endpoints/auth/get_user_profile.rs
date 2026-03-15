use std::sync::Arc;

use crate::domain::{entities::AuthClaims, errors::DomainError};
use crate::{error::AppResult, state::AppState};
use axum::extract::State;
use axum::{Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ProfileResponse {
    pub user_id: Option<String>,
    pub email: String,
    pub name: String,
    pub picture: String,
    pub department_id: Option<String>,
    pub is_manager: Option<bool>,
    pub is_admin: Option<bool>,
}

pub async fn get_user_profile(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
) -> AppResult<Json<ProfileResponse>> {
    let user_profile_data = match state
        .use_cases
        .get_user_profile
        .execute(claims.email.as_str())
        .await
    {
        Ok(data) => Some(data),
        Err(DomainError::NotFound(_)) => None,
        Err(err) => return Err(err.into()),
    };

    Ok(Json(ProfileResponse {
        user_id: user_profile_data.as_ref().map(|profile| profile.user_id.clone()),
        email: claims.email,
        name: claims.name,
        picture: claims.picture,
        department_id: user_profile_data
            .as_ref()
            .map(|profile| profile.department_id.clone()),
        is_manager: user_profile_data.as_ref().map(|profile| profile.is_manager),
        is_admin: user_profile_data.as_ref().map(|profile| profile.is_admin),
    }))
}
