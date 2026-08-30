// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

//! Memoria compartida del servidor (Gemelo Digital).

use crate::domain::telemetry::TelemetryPayload;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};

/// Estado global de la aplicación inyectado en las rutas de Axum.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Buffer en RAM de las últimas lecturas de telemetría de cada sensor.
    /// Funciona como el gemelo digital en tiempo real de la flota física.
    pub digital_twin: Arc<RwLock<HashMap<String, TelemetryPayload>>>,
    /// Pool de conexiones a la base de datos (`TimescaleDB`).
    pub db_pool: PgPool,
    /// Cola mpsc para escritura en base de datos asíncrona.
    pub tx_db: mpsc::Sender<TelemetryPayload>,
    /// Canal broadcast para notificar telemetría en tiempo real a `WebSockets`.
    pub tx_ws: broadcast::Sender<TelemetryPayload>,
}

impl AppState {
    /// Inicializa un nuevo estado global con el pool de base de datos inyectado.
    #[must_use]
    pub fn new(
        db_pool: PgPool,
        tx_db: mpsc::Sender<TelemetryPayload>,
        tx_ws: broadcast::Sender<TelemetryPayload>,
    ) -> Self {
        Self { digital_twin: Arc::new(RwLock::new(HashMap::new())), db_pool, tx_db, tx_ws }
    }
}
