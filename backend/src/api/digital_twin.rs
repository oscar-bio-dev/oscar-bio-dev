// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
// Proprietary and confidential.

use crate::domain::state::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::env;

/// Valida el API Key desde el header Authorization
pub fn validate_api_key(headers: &HeaderMap) -> Result<(), (StatusCode, String)> {
    let expected_key = env::var("API_KEY").unwrap_or_else(|_| "default_secure_key_123".to_string());

    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    if auth_header != format!("Bearer {expected_key}") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid API Key".to_string()));
    }

    Ok(())
}

/// Endpoint para leer el estado actual del Gemelo Digital en memoria.
#[utoipa::path(
    get,
    path = "/api/digital-twin",
    security(
        ("bearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Estado en tiempo real de todos los biosensores", body = std::collections::HashMap<String, shared::TelemetryPayload>),
        (status = 401, description = "No autorizado - API Key faltante o inválida")
    )
)]
#[allow(clippy::unused_async)]
pub async fn get_digital_twin(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    validate_api_key(&headers)?;

    let state_map = state.digital_twin.read().await.clone();

    Ok(Json(state_map))
}
