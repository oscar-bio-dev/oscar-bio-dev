// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
// Proprietary and confidential.

//! Endpoints para Kubernetes Probes (Liveness & Readiness)

use crate::domain::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "El servidor está vivo")
    )
)]
/// Endpoint de liveness probe para Kubernetes.
/// Devuelve 200 OK si el servidor HTTP está respondiendo.
#[allow(clippy::unused_async)]
pub async fn liveness_probe() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "El servidor y la base de datos están listos"),
        (status = 503, description = "Base de datos desconectada")
    )
)]
/// Endpoint de readiness probe para Kubernetes.
/// Devuelve 200 OK si el servidor y la base de datos están listos.
#[allow(clippy::unused_async)]
pub async fn readiness_probe(State(state): State<AppState>) -> impl IntoResponse {
    // Verificamos que el pool de conexiones no esté cerrado
    if state.db_pool.is_closed() {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "status": "down", "reason": "db_pool_closed" })),
        )
    } else {
        (StatusCode::OK, Json(serde_json::json!({ "status": "up" })))
    }
}
