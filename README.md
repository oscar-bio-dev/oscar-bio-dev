# Oscar Mora - Digital Portfolio & Conservation Technology Hub

Este repositorio alberga el portafolio digital y entorno de desarrollo interactivo de **Oscar Mora**, Biólogo Marino e Ingeniero Embebido (Conservation Technologist).

## 🚀 Misión y Arquitectura (Grado Espacial / Industrial)

Este proyecto no es un simple sitio web. Es un **Hub de Ingeniería y Telemetría** diseñado desde cero con filosofía *bare-metal* y *zero-cost abstractions* usando **Rust**. La meta es construir una plataforma interactiva, concurrente y de grado industrial para la visualización de datos de sensores ambientales (TinyML, RS485, IoT) en tiempo real, sirviendo además como currículum vivo.

---

## 🗺️ Roadmap de Ingeniería

Para asegurar una calidad implacable y cumplir con estándares industriales, el desarrollo de la plataforma está dividido en las siguientes fases:

### 🟢 Fase 1: Ignición y Rigor (Fundamentos)
- [x] **Configuración del entorno:** Cimientos de titanio con `rust-toolchain.toml`, reglas estrictas en `.rustfmt.toml` y configuraciones avanzadas para `rust-analyzer`.
- [x] **Validación de tipos y seguridad:** Reglas pedantes de Clippy (`clippy::all`, `clippy::pedantic`, `clippy::nursery`).
- [x] **Core del Router:** Implementación base usando `axum` sobre `tokio`.
- [x] **Static Server:** Servicio de archivos estáticos (CSS/assets) vía `tower-http`.
- [x] **Auditoría y CI/CD:** Hooks de pre-commit, `cargo-audit` (detección de vulnerabilidades), `cargo-deny` (licencias) y pipelines de GitHub Actions.
- [x] **Testing Básico:** Integración de `cargo test` como requisito obligatorio en el CI/CD para la validación continua de la lógica base.

### 🟡 Fase 2: Estructura Estática y Contratos API
- [ ] **Modularización Arquitectónica:** Patrón de Arquitectura Limpia (Clean Architecture), dividiendo endpoints en dominios lógicos (ej. `/api/telemetry`, `/portfolio`).
- [ ] **Motor de Plantillas SSR:** Jerarquía de layouts compilados en el binario usando `askama` (cero parsing en tiempo de ejecución).
- [ ] **Gestión de Errores Tipificada:** Uso de `thiserror` para el dominio y `anyhow` para la aplicación, con mapeo automático a HTTP status codes.
- [ ] **Contratos de API (OpenAPI):** Generación automática de especificaciones Swagger/OpenAPI usando `utoipa` para documentar la ingesta de sensores.
- [ ] **Documentación Interna (rustdoc):** Uso estricto de doc-comments (`///`) en estructuras y funciones públicas para generar manuales técnicos vivos automáticos con `cargo doc`.

### 🟠 Fase 3: Telemetría, Estado y Resiliencia
- [ ] **Observabilidad Industrial:** Logs estructurados y asíncronos mediante `tracing`, con miras a exportar a **OpenTelemetry (OTel)** (Prometheus/Grafana).
- [ ] **Estado Concurrente Compartido:** Inyección de dependencias seguras (`State`, `Arc`, `RwLock`) en Axum para manejar el estado global de la flota de sensores.
- [ ] **Graceful Shutdown:** Intercepción de señales del sistema (SIGINT/SIGTERM) para un apagado seguro sin pérdida de telemetría en vuelo.
- [ ] **Protección de Tráfico:** Middlewares de `tower` para *Rate Limiting* y *Timeout*, evitando saturación por ráfagas de datos de hardware defectuoso.
- [ ] **Pruebas Rigurosas (Mocking & Fuzzing):** Uso de `proptest` (Property-Based Testing) contra los endpoints y `mockall` para simular y testear el comportamiento sin el hardware físico conectado.

### 🔵 Fase 4: Persistencia y Flujo Time-Series
- [ ] **Capa de Base de Datos:** `sqlx` asíncrono con comprobación de queries en tiempo de compilación.
- [ ] **Almacenamiento Geoespacial/Temporal:** Preparación para PostgreSQL con extensiones **PostGIS** y **TimescaleDB** (ideal para modelado y retención de series temporales ambientales).
- [ ] **Infraestructura como Código (IaC):** Levantamiento determinista y orquestación de la base de datos usando `docker-compose` (local) y Terraform/Nix (para despliegues en producción).
- [ ] **Tolerancia a Fallos y Retención:** Implementación de un buffer de retención de datos (colas de mensajes como canales asíncronos de Tokio o Redis) para no perder ráfagas de datos críticos en caso de saturación o caída del DB.
- [ ] **Ingesta IoT Robusta:** Deserialización ultra-rápida (`serde`) y validación estricta de payloads (`validator`) antes de tocar la base de datos.
- [ ] **Protocolos Bidireccionales:** Implementación de WebSockets o puente HTTP-to-MQTT para control remoto y configuración *Over-The-Air* (OTA) de los biosensores.

### 🟣 Fase 5: Edge AI e Inferencia (TinyML)
- [ ] **Motor de Inferencia Backend:** Integración de modelos de Machine Learning (exportados de Edge Impulse) directamente en Rust usando `tract` u `ort`.
- [ ] **Clasificación en Tiempo Real:** Análisis de datos de calidad de agua/suelo en el servidor para generar alertas inmediatas antes de persistir.
- [ ] **Gemelo Digital (Digital Twin):** Representación virtual en memoria del estado actual de cada dispositivo de hardware (RP2350, ESP32) desplegado en campo.

### ⚫ Fase 6: Grado Espacial (Orbital & Wasm)
- [ ] **Frontend Reactivo Wasm:** Interfaz gráfica interactiva (dashboard) compilada a **WebAssembly** (ej. Leptos o Dioxus), eliminando frameworks JS externos.
- [ ] **Optimización Extrema:** Binarios en modo release optimizados con LTO (*Link Time Optimization*), `strip` de símbolos y `opt-level = "z"`.
- [ ] **Contenedorización Distroless:** Creación de imágenes Docker ultra-ligeras (`scratch` o `distroless`, < 15MB) blindadas contra vulnerabilidades del sistema operativo.
- [ ] **Seguridad de Hardware (mTLS):** Autenticación TLS mutua usando `rustls` para garantizar que solo sensores con certificados criptográficos válidos puedan publicar datos.

---

## 🛠️ Entorno de Trabajo Local

Este repositorio está configurado con las reglas más estrictas. Para contribuir o levantar el proyecto:

1. **Herramientas requeridas:** Rust (vía `rustup`). La versión (`stable`) y los componentes se fijarán automáticamente gracias a `rust-toolchain.toml`.
2. **Ejecución del servidor en modo desarrollo:**
   ```bash
   cargo run
   ```
3. **Revisión estática obligatoria (Linting):**
   ```bash
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```
4. **Formateo del Código:**
   ```bash
   cargo fmt --all -- --check
   ```
