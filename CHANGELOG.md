# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Phase 6 (Space Grade) Preparations**: Roadmap finalized for distroless Wasm deployments.

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
