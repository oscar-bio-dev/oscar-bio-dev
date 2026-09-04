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
            let mut attempts = 0;
            let mut delay = tokio::time::Duration::from_millis(500);

            loop {
                let query_result = sqlx::query(
                    r"
                    INSERT INTO telemetry (
                        device_id, timestamp, temperature, humidity, ph, dissolved_oxygen,
                        pressure, gas_resistance, co2, iaq, pm1_0, pm2_5, pm10_0,
                        battery_mv, sleep_cycles
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                    ",
                )
                .bind(&payload.device_id)
                .bind(payload.timestamp)
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
                .execute(&db_pool)
                .await;

                match query_result {
                    Ok(_) => {
                        tracing::debug!(
                            "DB Worker: Telemetría de {} persistida.",
                            payload.device_id
                        );
                        break;
                    }
                    Err(e) => {
                        attempts += 1;
                        if attempts >= 5 {
                            tracing::error!(
                                "CRITICAL: DB Worker falló al insertar telemetría tras 5 intentos. Datos descartados permanentemente. Device: {}, Payload: {:?}, Error: {}",
                                payload.device_id,
                                payload,
                                e
                            );
                            break;
                        }
                        tracing::warn!(
                            "DB Worker: Fallo temporal insertando telemetría de {}. Reintentando en {:?}... Error: {}",
                            payload.device_id, delay, e
                        );
                        tokio::time::sleep(delay).await;
                        delay *= 2; // Exponential backoff
                        if delay > tokio::time::Duration::from_secs(8) {
                            delay = tokio::time::Duration::from_secs(8);
                        }
                    }
                }
            }
        }

        tracing::warn!("DB Worker detenido: el canal mpsc de ingesta se ha cerrado.");
    });
}
