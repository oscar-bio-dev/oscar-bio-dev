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
        backend::api::telemetry::ingest_telemetry,
        backend::api::telemetry::ingest_telemetry_protobuf,
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
            backend::api::chat::ChatRequest,
            backend::api::chat::ChatResponse
        )
    ),tags((name = "telemetry", description = "Endpoints para sensores ambientales")),
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

use backend::domain::state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tower::timeout::TimeoutLayer;
use tower_http::limit::RequestBodyLimitLayer;

use axum::{error_handling::HandleErrorLayer, BoxError};

use backend::infrastructure::db::init_db_pool;

use axum_server::tls_rustls::RustlsConfig;
use rustls::server::WebPkiClientVerifier;
use rustls::RootCertStore;
use rustls::ServerConfig;

fn build_mtls_config() -> RustlsConfig {
    let mut root_store = RootCertStore::empty();
    let ca_file = std::fs::File::open("certs/ca.crt").expect("No se encontró certs/ca.crt");
    let mut ca_reader = std::io::BufReader::new(ca_file);
    let certs = rustls_pemfile::certs(&mut ca_reader).filter_map(Result::ok);
    for cert in certs {
        root_store.add(cert).unwrap();
    }

    let client_auth =
        WebPkiClientVerifier::builder(root_store.into()).allow_unauthenticated().build().unwrap();

    let cert_file =
        std::fs::File::open("certs/server.crt").expect("No se encontró certs/server.crt");
    let mut cert_reader = std::io::BufReader::new(cert_file);
    let cert_chain = rustls_pemfile::certs(&mut cert_reader).filter_map(Result::ok).collect();

    let key_file =
        std::fs::File::open("certs/server.key").expect("No se encontró certs/server.key");
    let mut key_reader = std::io::BufReader::new(key_file);
    let key = rustls_pemfile::private_key(&mut key_reader)
        .expect("No se pudo leer la llave")
        .expect("No key found");

    let mut server_config = ServerConfig::builder()
        .with_client_cert_verifier(client_auth)
        .with_single_cert(cert_chain, key)
        .expect("Mala configuración mTLS");

    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    RustlsConfig::from_config(Arc::new(server_config))
}

#[tokio::main]
async fn main() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Cargar variables de entorno
    let _ = dotenvy::dotenv();

    // Inicializamos el suscriptor de tracing con EnvFilter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Inicializar Pool de Base de Datos
    let db_pool = init_db_pool().await.expect("Fallo al conectar a TimescaleDB");

    // Ejecutar migraciones automáticamente
    tracing::info!("Verificando migraciones SQL...");
    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Fallo al migrar la DB");

    // Inicializar canales de buffer asíncrono y streaming WebSockets
    let (tx_db, rx_db) = tokio::sync::mpsc::channel(10_000);
    let (tx_ws, _rx_ws) = tokio::sync::broadcast::channel(100);

    // Iniciar el worker de persistencia en background
    backend::infrastructure::worker::start_db_worker(db_pool.clone(), rx_db);

    // Estado concurrente
    let app_state = AppState::new(db_pool, tx_db, tx_ws);

    // Definimos las rutas y la carpeta de archivos estáticos (assets)
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/telemetry", axum::routing::post(backend::api::telemetry::ingest_telemetry))
        .route(
            "/api/telemetry/protobuf",
            axum::routing::post(backend::api::telemetry::ingest_telemetry_protobuf),
        )
        .route(
            "/api/digital-twin",
            axum::routing::get(backend::api::digital_twin::get_digital_twin),
        )
        .route("/api/chat", axum::routing::post(backend::api::chat::chat_with_twin))
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
                .layer(TimeoutLayer::new(Duration::from_secs(30))),
        )
        .with_state(app_state);

    // Arrancamos el servidor
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let mut host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    if host == "localhost" {
        host = "127.0.0.1".to_string();
    }

    let addr = format!("{host}:{port}");
    let socket_addr: std::net::SocketAddr = addr.parse().expect("Formato de HOST IP inválido");

    let tls_config = build_mtls_config();

    tracing::info!("Servidor mTLS corriendo en https://{}", addr);

    axum_server::bind_rustls(socket_addr, tls_config)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}
