# Oscar Mora - Digital Portfolio & Conservation Technology Hub

This repository hosts the digital portfolio and interactive development environment of **Oscar Mora**, Marine Biologist and Embedded Engineer (Conservation Technologist).

## 🚀 Mission and Architecture (Space / Industrial Grade)

This project is not a simple website. It is an **Engineering and Telemetry Hub** designed from scratch with a *bare-metal* and *zero-cost abstractions* philosophy using **Rust**. The goal is to build an interactive, concurrent, and industrial-grade platform for real-time visualization of environmental sensor data (TinyML, RS485, IoT), while simultaneously serving as a living resume.

---

## 🗺️ Engineering Roadmap

To ensure relentless quality and comply with industrial standards, the platform's development is divided into the following phases:

### 🟢 Phase 1: Ignition and Rigor (Foundations)
- [x] **Environment Setup:** Titanium foundations with `rust-toolchain.toml`, strict rules in `.rustfmt.toml`, and advanced configurations for `rust-analyzer`.
- [x] **Type Safety and Validation:** Pedantic Clippy rules (`clippy::all`, `clippy::pedantic`, `clippy::nursery`).
- [x] **Router Core:** Base implementation using `axum` on top of `tokio`.
- [x] **Static Server:** Static file serving (CSS/assets) via `tower-http`.
- [x] **Auditing and CI/CD:** Pre-commit hooks, `cargo-audit` (vulnerability detection), `cargo-deny` (licenses), and GitHub Actions pipelines.
- [x] **Basic Testing:** Integration of `cargo test` as a mandatory requirement in the CI/CD pipeline for continuous validation of the core logic.

### 🟡 Phase 2: Static Structure and API Contracts
- [x] **Architectural Modularization:** Clean Architecture pattern, dividing endpoints into logical domains (e.g., `/api/telemetry`, `/portfolio`).
- [ ] **SSR Template Engine:** Hierarchy of layouts compiled directly into the binary using `askama` (zero parsing at runtime).
- [x] **Typed Error Handling:** Use of `thiserror` for the domain and `anyhow` for the application, with automatic mapping to HTTP status codes.
- [ ] **API Contracts (OpenAPI):** Automatic generation of Swagger/OpenAPI specifications using `utoipa` to document sensor ingestion.
- [ ] **Internal Documentation (rustdoc):** Strict use of doc-comments (`///`) on public structs and functions to generate live technical manuals automatically with `cargo doc`.

### 🟠 Phase 3: Telemetry, State, and Resilience
- [ ] **Industrial Observability:** Structured and asynchronous logging via `tracing`, aiming to export to **OpenTelemetry (OTel)** (Prometheus/Grafana).
- [ ] **Concurrent Shared State:** Safe dependency injection (`State`, `Arc`, `RwLock`) in Axum to handle the global state of the sensor fleet.
- [ ] **Graceful Shutdown:** Interception of system signals (SIGINT/SIGTERM) for a safe shutdown without losing telemetry data in flight.
- [ ] **Traffic Protection:** `tower` middlewares for *Rate Limiting* and *Timeout*, preventing saturation from burst data sent by faulty hardware.
- [ ] **Rigorous Testing (Mocking & Fuzzing):** Use of `proptest` (Property-Based Testing) against endpoints and `mockall` to mock and test behavior without physical hardware connected.

### 🔵 Phase 4: Persistence and Time-Series Flow
- [ ] **Database Layer:** Asynchronous `sqlx` with compile-time query verification.
- [ ] **Geospatial/Temporal Storage:** Preparation for PostgreSQL with **PostGIS** and **TimescaleDB** extensions (ideal for environmental time-series modeling and retention).
- [ ] **Infrastructure as Code (IaC):** Deterministic spin-up and orchestration of the database using `docker-compose` (local) and Terraform/Nix (for production deployments).
- [ ] **Fault Tolerance and Retention:** Implementation of a data retention buffer (message queues like Tokio async channels or Redis) to avoid losing critical burst data during database saturation or crashes.
- [ ] **Robust IoT Ingestion:** Ultra-fast deserialization (`serde`) and strict payload validation (`validator`) before touching the database.
- [ ] **Bidirectional Protocols:** Implementation of WebSockets or an HTTP-to-MQTT bridge for remote control and *Over-The-Air* (OTA) configuration of biosensors.

### 🟣 Phase 5: Edge AI and Inference (TinyML)
- [ ] **Backend Inference Engine:** Integration of Machine Learning models (exported from Edge Impulse) directly into Rust using `tract` or `ort`.
- [ ] **Real-Time Classification:** Server-side analysis of water/soil quality data to generate immediate alerts before persistence.
- [ ] **Digital Twin:** In-memory virtual representation of the current state of each hardware device (RP2350, ESP32) deployed in the field.

### ⚫ Phase 6: Space Grade (Orbital & Wasm)
- [ ] **Reactive Wasm Frontend:** Interactive graphical interface (dashboard) compiled to **WebAssembly** (e.g., Leptos or Dioxus), eliminating external JS frameworks.
- [ ] **Extreme Optimization:** Release binaries optimized with LTO (*Link Time Optimization*), symbol `strip`, and `opt-level = "z"`.
- [ ] **Distroless Containerization:** Creation of ultra-lightweight Docker images (`scratch` or `distroless`, < 15MB) shielded against OS-level vulnerabilities.
- [ ] **Hardware Security (mTLS):** Mutual TLS authentication using `rustls` to ensure that only sensors with valid cryptographic certificates can publish data.

---

## 🛠️ Local Development Environment

This repository is configured with the strictest rules. To contribute or spin up the project:

1. **Required tools:** Rust (via `rustup`). The version (`stable`) and components will be automatically pinned thanks to `rust-toolchain.toml`.
2. **Running the server in development mode:**
   ```bash
   cargo run
   ```
3. **Mandatory static analysis (Linting):**
   ```bash
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```
4. **Code Formatting:**
   ```bash
   cargo fmt --all -- --check
   ```
