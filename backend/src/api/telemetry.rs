// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

use crate::domain::state::AppState;
use axum::{body::Bytes, extract::State, http::StatusCode, response::IntoResponse, Json};
use prost::Message;
use shared::{TelemetryPayload, TelemetryPayloadPb};
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

    // 2. Actualizamos el Gemelo Digital en RAM
    {
        let mut twin = state.digital_twin.write().await;
        twin.insert(payload.device_id.clone(), payload.clone());
    }

    // 3. Persistencia Asíncrona (Buffer) y Streaming (WebSockets)
    let _ = state.tx_ws.send(payload.clone()); // Ignoramos si no hay clientes conectados

    if let Err(e) = state.tx_db.send(payload.clone()).await {
        tracing::error!(
            "Buffer de persistencia lleno o cerrado. Se perdió telemetría de {}: {}",
            payload.device_id,
            e
        );
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "Persistencia temporalmente inhabilitada (Buffer saturado)".to_string(),
        ));
    }

    tracing::info!(
        "Recibida telemetría HTTP desde {}: Temp = {}°C (Encolada)",
        payload.device_id,
        payload.temperature.value()
    );

    Ok((StatusCode::ACCEPTED, "Telemetría encolada correctamente"))
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

    // 3. Actualizamos el Gemelo Digital en RAM
    {
        let mut twin = state.digital_twin.write().await;
        twin.insert(payload.device_id.clone(), payload.clone());
    }

    // 4. Persistencia Asíncrona (Buffer) y Streaming (WebSockets)
    let _ = state.tx_ws.send(payload.clone());

    if let Err(e) = state.tx_db.send(payload.clone()).await {
        tracing::error!(
            "Buffer de persistencia lleno. Se perdió telemetría PROTOBUF de {}: {}",
            payload.device_id,
            e
        );
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "Persistencia temporalmente inhabilitada (Buffer saturado)".to_string(),
        ));
    }

    tracing::info!(
        "Recibida telemetría PROTOBUF desde {}: Temp = {}°C (Encolada)",
        payload.device_id,
        payload.temperature.value()
    );

    Ok((StatusCode::ACCEPTED, "Telemetría Protobuf encolada correctamente"))
}
