// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
// Proprietary and confidential.

//! Endpoint para ingesta de eventos de salud del Edge Telemetry Gateway.

use crate::domain::state::AppState;
use axum::{body::Bytes, extract::State, http::StatusCode, response::IntoResponse, Json};
use prost::Message;
use shared::{GatewayHealthEvent, GatewayHealthEventPb};
use validator::Validate;

/// Endpoint para recibir eventos de diagnóstico del Gateway (JSON).
#[utoipa::path(
    post,
    path = "/api/gateway-health",
    request_body = GatewayHealthEvent,
    responses(
        (status = 202, description = "Evento de salud del Gateway recibido"),
        (status = 400, description = "Payload inválido"),
        (status = 500, description = "Error interno de persistencia")
    )
)]
#[allow(clippy::unused_async)]
pub async fn ingest_gateway_health(
    State(state): State<AppState>,
    Json(event): Json<GatewayHealthEvent>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = event.validate() {
        tracing::error!("Fallo en validación de GatewayHealthEvent: {}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Invalid event: {e}")));
    }

    persist_gateway_health(&state, &event).await?;

    if event.is_degraded_mode {
        tracing::warn!(
            "⚠️  Gateway {} en MODO DEGRADADO: {}",
            event.gateway_id,
            event.alert_message.as_deref().unwrap_or("sin mensaje")
        );
    } else {
        tracing::info!(
            "Gateway {} saludable (uptime: {}s, heap: {} bytes)",
            event.gateway_id,
            event.uptime_seconds,
            event.free_heap_bytes
        );
    }

    Ok((StatusCode::ACCEPTED, "Evento de salud del Gateway recibido"))
}

/// Endpoint para recibir eventos de diagnóstico del Gateway (Protobuf binario).
#[utoipa::path(
    post,
    path = "/api/gateway-health/protobuf",
    request_body(content = [u8], content_type = "application/x-protobuf", description = "Binary Protobuf payload"),
    responses(
        (status = 202, description = "Evento de salud binario recibido"),
        (status = 400, description = "Payload binario inválido"),
        (status = 500, description = "Error interno de persistencia")
    )
)]
#[allow(clippy::unused_async)]
pub async fn ingest_gateway_health_protobuf(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pb_dto = GatewayHealthEventPb::decode(body).map_err(|e| {
        tracing::error!("Fallo al decodificar GatewayHealthEvent Protobuf: {}", e);
        (StatusCode::BAD_REQUEST, "Protobuf mal formado".to_string())
    })?;

    let event: GatewayHealthEvent = pb_dto.into();

    if let Err(e) = event.validate() {
        tracing::error!("Fallo en validación de GatewayHealthEvent (Protobuf): {}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Invalid event: {e}")));
    }

    persist_gateway_health(&state, &event).await?;

    tracing::info!(
        "GatewayHealth Protobuf recibido de {} (degraded={})",
        event.gateway_id,
        event.is_degraded_mode
    );

    Ok((StatusCode::ACCEPTED, "Evento de salud del Gateway (Protobuf) recibido"))
}

/// Persiste un `GatewayHealthEvent` directamente en `TimescaleDB`.
async fn persist_gateway_health(
    state: &AppState,
    event: &GatewayHealthEvent,
) -> Result<(), (StatusCode, String)> {
    sqlx::query(
        r"
        INSERT INTO gateway_health_events (
            gateway_id, timestamp, is_degraded_mode,
            sd_card_mounted, sd_card_total_mb, sd_card_free_mb, sd_io_errors,
            uptime_seconds, free_heap_bytes, alert_message
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ",
    )
    .bind(&event.gateway_id)
    .bind(event.timestamp)
    .bind(event.is_degraded_mode)
    .bind(event.sd_card_mounted)
    .bind(event.sd_card_total_mb.map(|v| i32::try_from(v).unwrap_or(i32::MAX)))
    .bind(event.sd_card_free_mb.map(|v| i32::try_from(v).unwrap_or(i32::MAX)))
    .bind(i32::try_from(event.sd_io_errors).unwrap_or(i32::MAX))
    .bind(i32::try_from(event.uptime_seconds).unwrap_or(i32::MAX))
    .bind(i32::try_from(event.free_heap_bytes).unwrap_or(i32::MAX))
    .bind(&event.alert_message)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Fallo al persistir GatewayHealthEvent: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error al persistir evento de salud".to_string())
    })?;

    Ok(())
}
