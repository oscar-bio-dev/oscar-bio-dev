# Workspace Agent Rules & Constraints (oscar-bio-dev)

This document defines the strict, industrial-grade behavioral rules, architectural guidelines, and repository governance standards that any AI agent must follow when interacting with this workspace. This repository (`oscar-bio-dev`) is exclusively dedicated to **Rust, Frontend (Wasm), Backend, Telemetry, Databases, and Web HMI**.

## 1. Industrial Space-Grade Systems Engineering
- **Rust Exclusivity**: All core logic, backend services, and web frontends must be written in Rust. Adhere to a *bare-metal* and *zero-cost abstractions* mindset.
- **Type Safety & Data Integrity**: 
  - Follow the "Parse, don't validate" paradigm using the `validator` crate.
  - Ensure lossless error propagation using `thiserror`.
  - Maintain the Hybrid Type Strategy: `f32` for Protobuf wire payloads (DTOs), and explicit casting to `f64` for Domain models and Database analytics.
- **Security & Stability**: 
  - 0 `unsafe` blocks unless explicitly authorized by a human principal engineer. 
  - No `panic!` or `unwrap()` in production code; handle all errors gracefully via `Result`.
  - Enforce poison-pill isolation: Any malformed telemetry must be immediately routed to a Dead-Letter Queue (DLQ) in PostgreSQL without crashing the async workers.

## 2. Cloud-Native & Decoupled Architecture
- **Ingestion Pipeline**: The backend does NOT expose direct TCP/mTLS ingestion ports to edge devices. It acts purely as a Pull Subscriber to Google Cloud Pub/Sub.
- **Idempotency**: All database insertions (TimescaleDB) must handle conflicts gracefully (e.g., `ON CONFLICT DO NOTHING`) using a composite unique key (`event_id`, `measured_at`).
- **Stateless Async Workers**: Use `tokio` for all asynchronous workloads. Do not block the executor thread with heavy computations or synchronous I/O.

## 3. Industrial HMI & Web Standards (Leptos/Wasm)
- **Extreme Fault Tolerance**: The UI must never freeze. Use WebSockets with automatic backoff reconnection logic. Handle network loss and component unmounting gracefully.
- **Terminal/Industrial Aesthetics**: Web applications must lean towards a sleek, functional industrial terminal aesthetic using the **Gruvbox Hard Dark** color palette. Avoid bloated whitespace.
- **Telemetry Staleness**: The frontend MUST visually represent the "age" of data. Telemetry older than 5 seconds must be visually attenuated (e.g., dimmed, grayscale) to indicate a stale state to the operator.
- **Modern & Lightweight**: Use **Leptos (WASM/CSR)** for the frontend. Rely on native CSS variables and modern HTML5 APIs. Avoid bloated JavaScript frameworks.

## 4. GitHub Workflow & Pull Request Policies
- **Branching Strategy**: 
  - `main` is strictly protected. **No direct pushes to main**.
  - Use semantic branch names: `feat/...`, `fix/...`, `docs/...`, `refactor/...`, `ci/...`.
- **Pull Request Rules**:
  - **1 PR = 1 Propósito**: PRs must be small, atomic, and focused on a single concern.
  - **PR Titles**: Must strictly follow Conventional Commits (e.g., `feat(ui): add telemetry staleness indicator`).
  - **PR Descriptions**: Must include context, changes made, risks, and how it was tested.
  - **Merge Blocks**: Do not merge if CI fails, there are compiler warnings, documentation is missing, or reviews are pending. Squash merge by default.
  - Delete merged branches promptly (within 7 days).

## 5. CI/CD & Security Compliance
- **Mandatory CI Checks (per PR)**:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
- **Security Audits**:
  - Run `cargo audit` to block vulnerable dependencies.
  - **No `.env` commits**. Use GitHub Secrets for environments.
  - Secret scanning and Dependabot must be enabled and alerts resolved immediately.

## 6. Repository Hygiene & Planning
- **Issue Tracking**: All work must be tracked through an Issue or documented task. Use consistent labels (`bug`, `feature`, `documentation`, `security`, `priority:high`).
- **Documentation Sync**: `README.md`, `docs/ROADMAP.md`, `CHANGELOG.md`, and architecture docs must be kept strictly synchronized with the shipped code state.
- **Releases**: Follow Semantic Versioning (SemVer), tag releases explicitly, and document breaking changes, fixes, and features concisely in `CHANGELOG.md`.
- **Proprietary IP**: Ensure every newly created source code file (`.rs`) contains the official SetaeSense copyright and confidentiality header.