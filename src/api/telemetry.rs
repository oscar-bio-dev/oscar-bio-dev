// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

use crate::domain::state::AppState;
use crate::domain::telemetry::TelemetryPayload;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

/// Endpoint para la ingesta de datos provenientes de los sensores de campo.
#[utoipa::path(
    post,
    path = "/api/telemetry",
    request_body = TelemetryPayload,
    responses(
        (status = 201, description = "Telemetría recibida correctamente")
    )
)]
#[allow(clippy::unused_async)]
pub async fn ingest_telemetry(
    State(state): State<AppState>,
    Json(payload): Json<TelemetryPayload>,
) -> impl IntoResponse {
    // Guardamos la lectura en la memoria compartida concurrente
    {
        let mut twin = state.digital_twin.write().await;
        twin.insert(payload.device_id.clone(), payload.clone());
    }

    // TODO: En la Fase 4 esto se insertará en TimescaleDB vía sqlx.
    tracing::info!(
        "Recibida telemetría válida desde {}: Temp = {}°C",
        payload.device_id,
        payload.temperature.value()
    );

    (StatusCode::CREATED, "Telemetría recibida correctamente")
}
