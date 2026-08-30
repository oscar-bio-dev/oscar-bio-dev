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

use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

// Askama compilará index.html directamente en el binario final
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
    role: &'static str,
}

// Controlador de la ruta principal
async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        title: "Oscar Mora | Portafolio",
        role: "Conservation Technologist & Embedded Engineer",
    };

    template.render().map_or_else(
        |_| axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        |html| Html(html).into_response(),
    )
}

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(oscar_bio_dev::api::telemetry::ingest_telemetry),
    components(schemas(
        oscar_bio_dev::domain::telemetry::TelemetryPayload,
        oscar_bio_dev::domain::telemetry::Temperature,
        oscar_bio_dev::domain::telemetry::Humidity,
        oscar_bio_dev::domain::telemetry::Ph,
        oscar_bio_dev::domain::telemetry::DissolvedOxygen
    )),
    tags((name = "telemetry", description = "Endpoints para sensores ambientales"))
)]
struct ApiDoc;

use oscar_bio_dev::domain::state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tower::timeout::TimeoutLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use axum::{error_handling::HandleErrorLayer, BoxError};

#[tokio::main]
async fn main() {
    // Inicializamos el suscriptor de tracing con EnvFilter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Estado concurrente
    let app_state = AppState::new();

    // Configuración de Rate Limiting
    let governor_conf =
        Arc::new(GovernorConfigBuilder::default().per_second(5).burst_size(10).finish().unwrap());
    let governor_layer = GovernorLayer { config: governor_conf };

    // Definimos las rutas y la carpeta de archivos estáticos (assets)
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(index))
        .route(
            "/api/telemetry",
            axum::routing::post(oscar_bio_dev::api::telemetry::ingest_telemetry),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            tower::ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        axum::http::StatusCode::REQUEST_TIMEOUT,
                        format!("Request dropped by protection layer: {err}"),
                    )
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .layer(governor_layer),
        )
        .with_state(app_state);

    // Arrancamos el servidor en el puerto 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Servidor Rust 'bare-metal' corriendo en http://localhost:3000");

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::info!("Señal de apagado recibida, iniciando Graceful Shutdown...");
}
