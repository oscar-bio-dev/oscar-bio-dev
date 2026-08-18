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

pub mod api;
pub mod domain;

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

#[tokio::main]
async fn main() {
    // Definimos las rutas y la carpeta de archivos estáticos (assets)
    let app = Router::new()
        .route("/", get(index))
        .route("/api/telemetry", axum::routing::post(api::telemetry::ingest_telemetry))
        .nest_service("/assets", ServeDir::new("assets"));

    // Arrancamos el servidor en el puerto 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor Rust 'bare-metal' corriendo en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
