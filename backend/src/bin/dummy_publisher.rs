// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.

use chrono::Utc;
use prost::Message;
use reqwest::Client;
use serde_json::json;
use shared::TelemetryPayloadPb;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let topic_url =
        "http://localhost:8085/v1/projects/oscar-bio-dev-project/topics/room-telemetry:publish";

    let mut sequence = 1;
    println!("Starting Dummy Publisher...");
    println!("Sending 21-field Protobuf payloads to {} every 2 seconds", topic_url);

    loop {
        let now_ms = Utc::now().timestamp_millis() as u64;

        let payload = TelemetryPayloadPb {
            protocol_version: 1,
            schema_version: 1,
            event_id: format!("dummy-evt-{}", sequence),
            gateway_id: "Gtw-Lab-01".to_string(),
            device_id: "ESP32-P4-Test".to_string(),
            node_sequence: sequence,
            measured_at_ms: now_ms - 100,
            ingested_at_ms: now_ms,
            temperature: Some(24.5 + (sequence as f32 % 5.0) * 0.1),
            humidity: Some(45.0 + (sequence as f32 % 10.0) * 0.5),
            ph: Some(7.2),
            dissolved_oxygen: Some(8.1),
            pressure: Some(1013.25),
            gas_resistance: Some(12000.0),
            co2: Some(400 + (sequence % 50)),
            iaq: Some(50.0),
            pm1_0: Some(10.0),
            pm2_5: Some(12.5),
            pm10_0: Some(15.0),
            battery_mv: Some(4100),
            sleep_cycles: Some(10),
        };

        let mut buf = Vec::new();
        payload.encode(&mut buf)?;

        // Encode payload as Base64 for the REST API
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let b64_data = STANDARD.encode(&buf);

        let body = json!({
            "messages": [
                {
                    "data": b64_data
                }
            ]
        });

        match client.post(topic_url).json(&body).send().await {
            Ok(res) if res.status().is_success() => {
                println!(
                    "Published Event {} -> CO2: {} ppm, Temp: {:.1} °C",
                    sequence,
                    payload.co2.unwrap(),
                    payload.temperature.unwrap()
                );
            }
            Ok(res) => {
                println!(
                    "Failed to publish. Status: {}, Body: {:?}",
                    res.status(),
                    res.text().await?
                );
            }
            Err(e) => {
                println!("Connection error: {}. Is the Pub/Sub emulator running on port 8085?", e);
            }
        }

        sequence += 1;
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
