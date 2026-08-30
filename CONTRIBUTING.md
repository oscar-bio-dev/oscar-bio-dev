# Contributing to oscar-bio-dev

First off, thank you for considering contributing to the `oscar-bio-dev` telemetry hub! We aim for space-grade, industrial-level code quality. 

## Development Process

1. **Fork the Repository:** Create your own branch from `main`.
2. **Environment Setup:** 
    - Install the Rust toolchain (we use stable).
    - Run `docker-compose up -d` to start the TimescaleDB/PostGIS database.
    - Copy `.env.example` to `.env` and fill in your keys.
    - Run `cargo sqlx database setup` to run migrations.
3. **Commit Standards:** We strictly follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
    - Example: `feat(telemetry): add support for new sensor type`
    - Example: `fix(api): resolve memory leak in digital twin`

## Quality Gates (Pre-requisites for PR)
Before opening a Pull Request, you MUST ensure the following checks pass locally:

- **Formatting:** Code must be formatted using `cargo fmt`.
  ```bash
  cargo fmt --all
  ```
- **Linting:** We enforce pedantic clippy rules. There must be zero warnings.
  ```bash
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  ```
- **Tests:** All unit tests must pass.
  ```bash
  cargo test
  ```
- **Security:** Run `cargo audit` to ensure no known vulnerabilities are introduced.

## Architectural Guidelines
- **Type Safety over Validation:** Parse, don't validate. Use Rust's type system to represent state.
- **Zero-Cost Abstractions:** Keep hot-paths (telemetry ingestion) allocation-free where possible.
- **No Unsafe:** Unless absolutely necessary and heavily documented.
