// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
// Proprietary and confidential.

//! Endpoint WebSocket para streaming de telemetría y configuración OTA (Downlink).

use crate::domain::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};

/// Comando de configuración (Over-The-Air Downlink) desde el frontend/backend hacia los sensores.
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "cmd")]
pub enum OtaCommand {
    /// Cambiar la tasa de muestreo.
    SetSampleRate {
        /// Frecuencia en milisegundos.
        rate_ms: u32,
    },
    /// Entrar en deep sleep por un tiempo determinado.
    DeepSleep {
        /// Duración en segundos.
        duration_s: u32,
    },
    /// Reiniciar el nodo.
    Reboot,
}

/// Endpoint para abrir la conexión WebSocket.
#[utoipa::path(
    get,
    path = "/api/ws",
    responses(
        (status = 101, description = "WebSocket upgrade exitoso")
    )
)]
#[allow(clippy::unused_async)]
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Nos suscribimos al canal de broadcast de telemetría
    let mut rx_telemetry = state.tx_ws.subscribe();

    let mut send_task = tokio::spawn(async move {
        loop {
            match rx_telemetry.recv().await {
                Ok(payload) => {
                    if let Ok(json_str) = serde_json::to_string(&payload) {
                        if sender.send(Message::Text(json_str)).await.is_err() {
                            break; // Cliente desconectado
                        }
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                    tracing::warn!("WebSocket client lagged behind. Skipped {} messages.", skipped);
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    break; // Canal cerrado
                }
            }
        }
    });

    // Tarea 2: Leer del cliente WebSocket (Comandos OTA JSON)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Intentamos parsear como OtaCommand
                    if let Ok(cmd) = serde_json::from_str::<OtaCommand>(&text) {
                        tracing::info!("OTA Command recibido: {:?}", cmd);
                        // Aquí se integraría con un puente MQTT para mandar el comando físico al nodo.
                    } else {
                        tracing::warn!("Mensaje de WS desconocido recibido: {}", text);
                    }
                }
                Message::Close(_) => {
                    tracing::debug!("Cliente WebSocket cerró la conexión.");
                    break;
                }
                _ => {}
            }
        }
    });

    // Si una de las dos tareas finaliza, abortamos la otra para limpiar la conexión.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
