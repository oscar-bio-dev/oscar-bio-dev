# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2026-09-04

### Added
- **Mega-Schema Convergence**: Backend Protobuf schema and domain models now aligned 1:1 with the canonical Gateway `TelemetryPayload` (15 fields).
- **New newtypes**: `Iaq` (0-500), `Pm1_0`, `Pm2_5`, `Pm10_0` (0-1000 µg/m³) with full domain validation.
- **Node diagnostics**: `battery_mv` and `sleep_cycles` fields propagated end-to-end from proto → Rust model → TimescaleDB.
- **Gateway Health Infrastructure (Contract-First)**:
  - New `gateway_health_events` TimescaleDB hypertable (migration `0004`).
  - New `GatewayHealthEvent` + `GatewayHealthEventPb` domain models.
  - New `POST /api/gateway-health` and `POST /api/gateway-health/protobuf` endpoints on mTLS strict port.
- **OpenAPI**: Registered `Iaq`, `Pm1_0`, `Pm2_5`, `Pm10_0`, `GatewayHealthEvent` schemas in Swagger UI.
- **Frontend**: Leptos Wasm dashboard now renders 12 sensor fields per node (IAQ, PM2.5, PM10, battery, sleep cycles, etc.).
- **SQL Migration `0003`**: Adds `iaq`, `pm1_0`, `pm2_5`, `pm10_0`, `battery_mv`, `sleep_cycles` columns to `telemetry` table.

### Changed
- **Hybrid f32/f64 Architecture**: Protobuf DTO uses `f32` (hardware wire type) with explicit `f64::from()` widening for domain model and DB persistence.
- **Temperature relaxed**: `temperature` field changed from required to `Option<Temperature>` to support heterogeneous sensor nodes.
- **DB Worker**: INSERT query expanded from 9 to 15 column binds with safe `u32→i32` conversion.
- **Device ID regex**: Extended to accept `:` and `.` characters for MAC-format IDs (`sensor-AA:BB:CC:DD:EE:FF`).

## [0.5.0] - 2026-09-01

### Added
- **Phase 3 Security Hardening**:
  - `LruCache` (cap 10,000) replaces unbounded `HashMap` for Digital Twin state to prevent `DoS` via RAM exhaustion.
  - Gemini API key moved from query param to `x-goog-api-key` HTTP header.
  - `system_instruction` schema for Gemini API to prevent prompt injection.
  - User message truncation to 2,000 characters.
  - DB Worker exponential backoff capped at 8 seconds with critical log on 5th failure.
  - WebSocket set to read-only mode (OTA command stub removed until mTLS auth layer is ready).

### Changed
- Startup error handling refactored from `.unwrap()`/`.expect()` to `Result<>` propagation in `main.rs`.

### Removed
- `backend/assets/chatbot.js` and `backend/assets/style.css` (legacy, contained hardcoded credentials).

## [0.4.5] - 2026-08-30

### Added
- **Phase 5: Edge & Cloud AI Synergy**:
  - Direct integration with Google Gemini 2.5 Flash LLM via `reqwest`.
  - Cognitive Digital Twin in-memory state mapped and sent as dynamic system prompts.
  - New `GET /api/digital-twin` endpoint to fetch the global telemetry state.
  - New `POST /api/chat` endpoint for AI interaction.
  - Custom Agent Terminal UI built into Askama templates with Vanilla JS and dynamic typing effects.
  - Basic API Key security header implementation (`Authorization: Bearer`).

### Fixed
- Upgraded `sqlx` from `0.7.4` to `0.8.6` to resolve critical vulnerabilities (`RUSTSEC-2024-0363`).
- Bumping `validator` to `0.19` resolving `idna` vulnerability (`RUSTSEC-2024-0421`).
- Added `.cargo/audit.toml` to manage transitive unresolved RUSTSEC warnings.

## [0.3.0] - 2026-08-28

### Added
- **Phase 4: Persistence and Time-Series Flow**:
  - Asynchronous Database layer configured with `sqlx`.
  - Database schema migrations added for TimescaleDB and PostGIS.
  - Docker Compose orchestration (`docker-compose.yml`) for local database spin-up.
  - Domain models expanded to include Air Quality metrics (BME688 / SCD41).

## [0.2.0] - 2026-08-15

### Added
- **Phase 3: Robust IoT Ingestion**:
  - `protobuf` schema definition (`proto/telemetry.proto`) for zero-cost binary serialization.
  - Endpoints generated and configured using `prost` and Axum.

## [0.1.0] - 2026-08-01

### Added
- **Phase 1 & 2: Project Initialization**:
  - Pure Rust Axum server setup.
  - Basic HTML rendering via `askama`.
  - Initial `README.md` and repository scaffolding.
