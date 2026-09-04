#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused
)]
#![allow(clippy::needless_for_each)]
// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::multiple_crate_versions
)]

// Modules are now imported from the oscar_bio_dev library crate.

// The Askama template has been replaced by the Leptos SPA in `frontend/dist`.
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        backend::api::digital_twin::get_digital_twin,
        backend::api::digital_twin::get_digital_twin,
        backend::api::chat::chat_with_twin
    ),
    components(
        schemas(
            shared::TelemetryPayload,
            shared::Temperature,
            shared::Humidity,
            shared::Ph,
            shared::DissolvedOxygen,
            shared::Pressure,
            shared::GasResistance,
            shared::Co2,
            shared::Iaq,
            shared::Pm1_0,
            shared::Pm2_5,
            shared::Pm10_0,
            shared::GatewayHealthEvent,
            backend::api::chat::ChatRequest,
            backend::api::chat::ChatResponse
        )
    ),tags(
        (name = "telemetry", description = "Endpoints para sensores ambientales"),
        (name = "diagnostics", description = "Eventos de salud del Edge Gateway")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("Token")
                        .build(),
                ),
            );
        }
    }
}

use axum::http::Method;
use backend::domain::state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tower::timeout::TimeoutLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use axum::{error_handling::HandleErrorLayer, BoxError};

use backend::infrastructure::db::init_db_pool;

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("Received Ctrl-C, shutting down gracefully...");
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Cargar variables de entorno
    let _ = dotenvy::dotenv();

    // Inicializamos el suscriptor de tracing con EnvFilter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Inicializar Pool de Base de Datos
    let db_pool = init_db_pool().await?;

    // Ejecutar migraciones automáticamente
    tracing::info!("Verificando migraciones SQL...");
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    // Inicializar canales de streaming WebSockets
    let (tx_ws, _rx_ws) = tokio::sync::broadcast::channel(100);

    // Estado concurrente
    let app_state = AppState::new(db_pool.clone(), tx_ws);

    // Iniciar el worker de Pub/Sub en background
    if let Err(e) = backend::infrastructure::pubsub::start_pubsub_worker(app_state.clone()).await {
        tracing::error!("No se pudo inicializar Pub/Sub Worker: {}", e);
    }

    // Configurar rate limiting para la API pública (ej. 2 requests por segundo, burst de 10)
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .ok_or("Invalid governor config")?,
    );

    // Chat API stricter rate limiting: ~5 requests per minute
    let chat_governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(12) // 1 request every 12 seconds = 5 req/min
            .burst_size(2)
            .finish()
            .ok_or("Invalid chat governor config")?,
    );

    let cors_layer = CorsLayer::new()
        .allow_origin("https://oscar-bio.dev".parse::<axum::http::HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let public_app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", axum::routing::get(backend::api::health::liveness_probe))
        .route("/ready", axum::routing::get(backend::api::health::readiness_probe))
        .route(
            "/api/digital-twin",
            axum::routing::get(backend::api::digital_twin::get_digital_twin),
        )
        .route(
            "/api/chat",
            axum::routing::post(backend::api::chat::chat_with_twin)
                .layer(GovernorLayer { config: chat_governor_conf }),
        )
        .route("/api/ws", axum::routing::get(backend::api::ws::ws_handler))
        .fallback_service(
            ServeDir::new("frontend/dist")
                .not_found_service(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(
            tower::ServiceBuilder::new()
                .layer(RequestBodyLimitLayer::new(16 * 1024)) // 16 KB max payload
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        axum::http::StatusCode::REQUEST_TIMEOUT,
                        format!("Request dropped by protection layer: {err}"),
                    )
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(GovernorLayer { config: governor_conf }),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .with_state(app_state.clone());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let mut host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    if host == "localhost" {
        host = "127.0.0.1".to_string();
    }

    let public_addr = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&public_addr).await?;

    tracing::info!("Servidor web público iniciado en http://{}", public_addr);

    // Shutdown gracefully
    let axum_server = axum::serve(listener, public_app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    axum_server.await?;

    Ok(())
}
