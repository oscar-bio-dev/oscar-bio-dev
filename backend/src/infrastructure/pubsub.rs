// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
// Proprietary and confidential.

//! Background worker (Subscriber) for GCP Pub/Sub.

use crate::domain::state::AppState;
use google_cloud_pubsub::client::Subscriber;
use prost::Message;
use shared::{TelemetryPayload, TelemetryPayloadPb};
use validator::Validate;

/// Starts the Pub/Sub subscriber in the background.
pub async fn start_pubsub_worker(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Inicializando Pub/Sub Subscriber...");

    // El client_builder automáticamente usará PUBSUB_EMULATOR_HOST si está definida.
    let client = Subscriber::builder().build().await?;

    let subscription_name = std::env::var("PUBSUB_SUBSCRIPTION_NAME").unwrap_or_else(|_| {
        "projects/oscar-bio-dev-project/subscriptions/room-telemetry-sub".to_string()
    });

    tracing::info!("Pub/Sub Worker configurado para escuchar en {}...", subscription_name);

    // Iniciamos la recepción de mensajes en background
    tokio::spawn(async move {
        let mut stream = client.subscribe(&subscription_name).build();

        while let Some(res) = stream.next().await {
            match res {
                Ok((message, handler)) => {
                    let msg_id = message.message_id.clone();
                    tracing::debug!("Recibido mensaje de Pub/Sub (ID: {})", msg_id);

                    // 1. Decodificar Protobuf
                    let pb_dto = match TelemetryPayloadPb::decode(message.data.as_ref()) {
                        Ok(dto) => dto,
                        Err(e) => {
                            tracing::error!(
                                "Poison Pill (Protobuf inválido). Msg ID: {}, Error: {}",
                                msg_id,
                                e
                            );
                            handle_poison_pill(
                                &state,
                                message.data.to_vec(),
                                "Invalid Protobuf",
                                None,
                            )
                            .await;
                            handler.ack();
                            continue;
                        }
                    };

                    // 2. Mapear a dominio
                    let payload: TelemetryPayload = match pb_dto.try_into() {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::error!(
                                "Poison Pill (Dominio inválido). Msg ID: {}, Error: {}",
                                msg_id,
                                e
                            );
                            handle_poison_pill(
                                &state,
                                message.data.to_vec(),
                                &format!("Domain validation failed: {e}"),
                                None,
                            )
                            .await;
                            handler.ack();
                            continue;
                        }
                    };

                    // 3. Validar payload (Validator)
                    if let Err(e) = payload.validate() {
                        tracing::error!(
                            "Poison Pill (Validación estricta fallida). Msg ID: {}, Error: {}",
                            msg_id,
                            e
                        );
                        handle_poison_pill(
                            &state,
                            message.data.to_vec(),
                            &format!("Strict validation failed: {e}"),
                            Some(&payload.event_id),
                        )
                        .await;
                        handler.ack();
                        continue;
                    }

                    let event_id = payload.event_id.clone();

                    // 4. Actualizamos el Gemelo Digital en RAM
                    {
                        let mut twin = state.digital_twin.write().await;
                        twin.put(payload.device_id.clone(), payload.clone());
                    }

                    // 5. Broadcast vía WebSockets
                    let _ = state.tx_ws.send(payload.clone());

                    // 6. Inserción directa en Base de Datos (con idempotencia)
                    match persist_telemetry(&state, &payload).await {
                        Ok(()) => {
                            tracing::info!("Mensaje {} persistido exitosamente (ACK)", msg_id);
                            handler.ack();
                        }
                        Err(e) => {
                            tracing::error!("Fallo de Infraestructura (TimescaleDB). Haciendo NACK del mensaje {} (Event: {}). Error: {}", msg_id, event_id, e);
                            handler.nack(); // El broker aplicará backoff exponencial
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Fallo al recibir mensaje de Pub/Sub stream: {}", e);
                }
            }
        }
    });

    Ok(())
}

/// Inserta la telemetría en `TimescaleDB`. Retorna error si la DB falla.
async fn persist_telemetry(
    state: &AppState,
    payload: &TelemetryPayload,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        INSERT INTO telemetry (
            event_id, protocol_version, schema_version, gateway_id, device_id, node_sequence,
            measured_at, ingested_at, temperature, humidity, ph, dissolved_oxygen,
            pressure, gas_resistance, co2, iaq, pm1_0, pm2_5, pm10_0,
            battery_mv, sleep_cycles
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
        ON CONFLICT (event_id, measured_at) DO NOTHING
        ",
    )
    .bind(&payload.event_id)
    .bind(i32::try_from(payload.protocol_version).unwrap_or(1))
    .bind(i32::try_from(payload.schema_version).unwrap_or(1))
    .bind(&payload.gateway_id)
    .bind(&payload.device_id)
    .bind(i32::try_from(payload.node_sequence).unwrap_or(0))
    .bind(payload.measured_at)
    .bind(payload.ingested_at)
    .bind(payload.temperature.map(shared::Temperature::value))
    .bind(payload.humidity.map(shared::Humidity::value))
    .bind(payload.ph.map(shared::Ph::value))
    .bind(payload.dissolved_oxygen.map(shared::DissolvedOxygen::value))
    .bind(payload.pressure.map(shared::Pressure::value))
    .bind(payload.gas_resistance.map(shared::GasResistance::value))
    .bind(payload.co2.map(shared::Co2::value))
    .bind(payload.iaq.map(shared::Iaq::value))
    .bind(payload.pm1_0.map(shared::Pm1_0::value))
    .bind(payload.pm2_5.map(shared::Pm2_5::value))
    .bind(payload.pm10_0.map(shared::Pm10_0::value))
    .bind(payload.battery_mv.map(|v| i32::try_from(v).unwrap_or(i32::MAX)))
    .bind(payload.sleep_cycles.map(|v| i32::try_from(v).unwrap_or(i32::MAX)))
    .execute(&state.db_pool)
    .await?;

    Ok(())
}

/// Guarda un payload inválido (Poison Pill) en la tabla DLQ.
/// Si falla la inserción a DLQ, solo registramos error en log para no saturar Pub/Sub con NACKs por mensajes intrínsecamente malos.
async fn handle_poison_pill(
    state: &AppState,
    raw_payload: Vec<u8>,
    error_reason: &str,
    event_id: Option<&str>,
) {
    let result = sqlx::query(
        r"
        INSERT INTO telemetry_dlq (raw_payload, error_reason, event_id)
        VALUES ($1, $2, $3)
        ",
    )
    .bind(raw_payload)
    .bind(error_reason)
    .bind(event_id)
    .execute(&state.db_pool)
    .await;

    if let Err(e) = result {
        tracing::error!(
            "CRITICAL: Falló la escritura en DLQ PostgreSQL. Poison Pill perdida. Error: {}",
            e
        );
    }
}
