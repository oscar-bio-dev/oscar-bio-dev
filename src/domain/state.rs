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
use tokio::sync::RwLock;

/// Estado global de la aplicación inyectado en las rutas de Axum.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Buffer en RAM de las últimas lecturas de telemetría de cada sensor.
    /// Funciona como el gemelo digital en tiempo real de la flota física.
    pub digital_twin: Arc<RwLock<HashMap<String, TelemetryPayload>>>,
    /// Pool de conexiones a la base de datos (`TimescaleDB`).
    pub db_pool: PgPool,
}

impl AppState {
    /// Inicializa un nuevo estado global con el pool de base de datos inyectado.
    #[must_use]
    pub fn new(db_pool: PgPool) -> Self {
        Self { digital_twin: Arc::new(RwLock::new(HashMap::new())), db_pool }
    }
}
