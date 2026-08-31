// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
// Proprietary and confidential.

//! Background workers para retención de datos y procesamiento asíncrono.

use shared::TelemetryPayload;
use sqlx::PgPool;
use tokio::sync::mpsc;

/// Inicia el worker en background para procesar la cola de persistencia.
/// Al procesar en background, liberamos el endpoint HTTP para que retorne instantáneamente.
pub fn start_db_worker(db_pool: PgPool, mut rx: mpsc::Receiver<TelemetryPayload>) {
    tokio::spawn(async move {
        tracing::info!("DB Worker asíncrono iniciado. Esperando telemetría...");

        while let Some(payload) = rx.recv().await {
            let query_result = sqlx::query(
                r"
                INSERT INTO telemetry (device_id, timestamp, temperature, humidity, ph, dissolved_oxygen, pressure, gas_resistance, co2)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ",
            )
            .bind(&payload.device_id)
            .bind(payload.timestamp)
            .bind(payload.temperature.value())
            .bind(payload.humidity.map(shared::Humidity::value))
            .bind(payload.ph.map(shared::Ph::value))
            .bind(payload.dissolved_oxygen.map(shared::DissolvedOxygen::value))
            .bind(payload.pressure.map(shared::Pressure::value))
            .bind(payload.gas_resistance.map(shared::GasResistance::value))
            .bind(payload.co2.map(shared::Co2::value))
            .execute(&db_pool)
            .await;

            if let Err(e) = query_result {
                // En un sistema industrial real, aquí podríamos insertar en un archivo local (WAL)
                // o reintentar (Retry Policy) en caso de fallo temporal de TimescaleDB.
                tracing::error!(
                    "DB Worker falló al insertar telemetría de {}: {}",
                    payload.device_id,
                    e
                );
            } else {
                tracing::debug!("DB Worker: Telemetría de {} persistida.", payload.device_id);
            }
        }

        tracing::warn!("DB Worker detenido: el canal mpsc de ingesta se ha cerrado.");
    });
}
