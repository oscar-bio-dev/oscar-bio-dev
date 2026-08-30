// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

use crate::domain::telemetry::TelemetryPayload;
use axum::{http::StatusCode, response::IntoResponse, Json};

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
pub async fn ingest_telemetry(Json(payload): Json<TelemetryPayload>) -> impl IntoResponse {
    // Gracias al tipado fuerte y #[serde(try_from)], sabemos que `payload` es 100% válido aquí.
    // No necesitamos escribir validaciones manuales repetitivas en el controlador.

    // TODO: En la Fase 4 esto se insertará en TimescaleDB vía sqlx.
    tracing::info!(
        "Recibida telemetría válida desde {}: Temp = {}°C",
        payload.device_id,
        payload.temperature.value()
    );

    (StatusCode::CREATED, "Telemetría recibida correctamente")
}
