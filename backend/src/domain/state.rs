// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

//! Memoria compartida del servidor (Gemelo Digital).

use lru::LruCache;
use shared::TelemetryPayload;
use sqlx::PgPool;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Estado global de la aplicación inyectado en las rutas de Axum.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Buffer en RAM de las últimas lecturas de telemetría de cada sensor.
    /// Funciona como el gemelo digital en tiempo real de la flota física.
    /// Limitado a 10,000 entradas para evitar `DoS` por agotamiento de RAM.
    pub digital_twin: Arc<RwLock<LruCache<String, TelemetryPayload>>>,
    /// Pool de conexiones a la base de datos (`TimescaleDB`).
    pub db_pool: PgPool,
    /// Canal broadcast para notificar telemetría en tiempo real a `WebSockets`.
    pub tx_ws: broadcast::Sender<TelemetryPayload>,
}

impl AppState {
    /// Inicializa un nuevo estado global con el pool de base de datos inyectado.
    #[must_use]
    pub fn new(db_pool: PgPool, tx_ws: broadcast::Sender<TelemetryPayload>) -> Self {
        Self {
            digital_twin: Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(10_000).unwrap()))),
            db_pool,
            tx_ws,
        }
    }
}
