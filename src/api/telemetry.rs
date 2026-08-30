// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

use crate::domain::state::AppState;
use crate::domain::telemetry::{TelemetryPayload, TelemetryPayloadPb};
use axum::{body::Bytes, extract::State, http::StatusCode, response::IntoResponse, Json};
use prost::Message;
use validator::Validate;

/// Endpoint para la ingesta de datos provenientes de los sensores de campo.
#[utoipa::path(
    post,
    path = "/api/telemetry",
    request_body = TelemetryPayload,
    responses(
        (status = 201, description = "Telemetría recibida correctamente"),
        (status = 400, description = "Payload inválido"),
        (status = 500, description = "Error interno de persistencia")
    )
)]
#[allow(clippy::unused_async)]
pub async fn ingest_telemetry(
    State(state): State<AppState>,
    Json(payload): Json<TelemetryPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Validación estricta con Validator
    if let Err(e) = payload.validate() {
        tracing::error!("Fallo en validación de payload: {}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Invalid payload: {e}")));
    }

    // 2. Persistencia determinista vía sqlx
    let query_result = sqlx::query(
        r"
        INSERT INTO telemetry (device_id, timestamp, temperature, humidity, ph, dissolved_oxygen, pressure, gas_resistance, co2)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ",
    )
    .bind(&payload.device_id)
    .bind(payload.timestamp)
    .bind(payload.temperature.value())
    .bind(payload.humidity.map(crate::domain::telemetry::Humidity::value))
    .bind(payload.ph.map(crate::domain::telemetry::Ph::value))
    .bind(payload.dissolved_oxygen.map(crate::domain::telemetry::DissolvedOxygen::value))
    .bind(payload.pressure.map(crate::domain::telemetry::Pressure::value))
    .bind(payload.gas_resistance.map(crate::domain::telemetry::GasResistance::value))
    .bind(payload.co2.map(crate::domain::telemetry::Co2::value))
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            // 3. Actualizamos el Gemelo Digital en RAM
            {
                let mut twin = state.digital_twin.write().await;
                twin.insert(payload.device_id.clone(), payload.clone());
            }

            tracing::info!(
                "Recibida telemetría persistida desde {}: Temp = {}°C",
                payload.device_id,
                payload.temperature.value()
            );

            Ok((StatusCode::CREATED, "Telemetría persistida correctamente"))
        }
        Err(e) => {
            tracing::error!("Error insertando en la base de datos: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno al procesar telemetría".to_string(),
            ))
        }
    }
}

/// Endpoint para la ingesta de telemetría binaria de ultra-bajo peso (Protobuf).
#[utoipa::path(
    post,
    path = "/api/telemetry/protobuf",
    request_body(content = [u8], content_type = "application/x-protobuf", description = "Binary Protobuf payload"),
    responses(
        (status = 201, description = "Telemetría binaria persistida correctamente"),
        (status = 400, description = "Payload binario inválido"),
        (status = 500, description = "Error interno de persistencia")
    )
)]
#[allow(clippy::unused_async)]
pub async fn ingest_telemetry_protobuf(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Decodificar el DTO binario de Protobuf
    let pb_dto = match TelemetryPayloadPb::decode(body) {
        Ok(dto) => dto,
        Err(e) => {
            tracing::error!("Fallo al decodificar Protobuf: {}", e);
            return Err((StatusCode::BAD_REQUEST, "Protobuf mal formado".to_string()));
        }
    };

    // 2. Mapear y validar estrictamente hacia nuestro Modelo de Dominio
    let payload: TelemetryPayload = match pb_dto.try_into() {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Fallo en validación de dominio (desde Protobuf): {}", e);
            return Err((StatusCode::BAD_REQUEST, format!("Invalid domain data: {e}")));
        }
    };

    // Validación adicional (ej. longitud del device_id usando validator)
    if let Err(e) = payload.validate() {
        tracing::error!("Fallo en validación adicional de payload Protobuf: {}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Invalid payload rules: {e}")));
    }

    // 3. Persistencia determinista vía sqlx
    let query_result = sqlx::query(
        r"
        INSERT INTO telemetry (device_id, timestamp, temperature, humidity, ph, dissolved_oxygen, pressure, gas_resistance, co2)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "
    )
    .bind(&payload.device_id)
    .bind(payload.timestamp)
    .bind(payload.temperature.value())
    .bind(payload.humidity.map(crate::domain::telemetry::Humidity::value))
    .bind(payload.ph.map(crate::domain::telemetry::Ph::value))
    .bind(payload.dissolved_oxygen.map(crate::domain::telemetry::DissolvedOxygen::value))
    .bind(payload.pressure.map(crate::domain::telemetry::Pressure::value))
    .bind(payload.gas_resistance.map(crate::domain::telemetry::GasResistance::value))
    .bind(payload.co2.map(crate::domain::telemetry::Co2::value))
    .execute(&state.db_pool)
    .await;

    match query_result {
        Ok(_) => {
            {
                let mut twin = state.digital_twin.write().await;
                twin.insert(payload.device_id.clone(), payload.clone());
            }

            tracing::info!(
                "Recibida telemetría PROTOBUF desde {}: Temp = {}°C",
                payload.device_id,
                payload.temperature.value()
            );

            Ok((StatusCode::CREATED, "Telemetría Protobuf persistida correctamente"))
        }
        Err(e) => {
            tracing::error!("Error insertando Protobuf en la DB: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno al procesar telemetría binaria".to_string(),
            ))
        }
    }
}
