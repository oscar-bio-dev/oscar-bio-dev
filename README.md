# Oscar Mora | Conservation Technologist & Embedded Systems Engineer

[![Portfolio](https://img.shields.io/badge/Website-oscar--bio.dev-2a9d8f?style=for-the-badge&logo=googlechrome&logoColor=white)](https://oscar-bio.dev/)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-oscar--bio--dev-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white)](https://linkedin.com/in/oscar-bio-dev)
[![GitHub](https://img.shields.io/badge/GitHub-oscar--bio--dev-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/oscar-bio-dev)
[![Rust](https://img.shields.io/badge/Rust-1.85+-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![C/C++](https://img.shields.io/badge/C%2FC%2B%2B-Embedded-00599C?style=for-the-badge&logo=c%2B%2B&logoColor=white)](https://isocpp.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

[English](README.md) • [Versión en Español](README.es.md)

---

## About Me

I am a **Conservation Technologist**, **Marine Biologist**, and **Embedded Systems Engineer** based in Pueblo West, Colorado. I bridge the gap between ecological research and bare-metal instrumentation, translating complex environmental requirements into low-power IoT monitoring nodes, resilient telemetry infrastructure, and data-driven analytical platforms.

As the founder of **[EcoWest Instrument](https://oscar-bio.dev/)**, I engineer ruggedized hardware, specialized AFE sensor transducers, and software platforms built to operate under harsh field conditions.

- **Website & Portfolio:** [https://oscar-bio.dev/](https://oscar-bio.dev/)
- **Core Specialization:** In-situ environmental sensing, low-power edge telemetry, distributed time-series architectures, bare-metal firmware, and applied edaphic biodiversity modeling.
- **Location:** Pueblo West, Colorado, United States.

---

## Featured Projects & Initiatives

### 📡 [Ecological Telemetry Hub](https://github.com/oscar-bio-dev/telemetry-hub)
Edge-to-cloud telemetry ingestion and Digital Twin platform built in Rust (`Axum`, `Tokio`) and WebAssembly (`Leptos`), backed by TimescaleDB and PostGIS.
- **Unified Mega-Schema (33 fields):** Type-safe Protobuf wire contract shared between C firmware (`nanopb`) and Rust services (`prost`).
- **Resilient Cloud Ingestion & DLQ:** Asynchronous GCP Pub/Sub pull subscriber with automatic poison-pill dead-letter queue routing.
- **Cognitive Digital Twin:** Real-time state cache (`Arc<RwLock<LruCache>>`) with AI contextual reasoning.
- **Zero JS Overhead HMI:** Industrial Leptos Wasm dashboard featuring Gruvbox Hard Dark aesthetic, live sensor diagnostics, and real-time WebSocket state streaming.

### 🔬 [EcoWest Instrument](https://oscar-bio.dev/)
Commercial and scientific instrumentation for soil and aquatic ecosystems:
- **Optical Dissolved Oxygen (WQM-V100):** High-precision optical DO luminescent lifetime transduction with zero CPU drift.
- **Multiparameter Environmental Nodes:** Low-power sensor nodes integrating Bosch Sensortec BME688 (IAQ, VOC), Sensirion SCD41 (NDIR CO₂), and Bosch BMV080 (laser particulate PM1.0/PM2.5/PM10).
- **Industrial Gateways:** Edge telemetry gateways powered by ESP32-P4, RP2350, and MSPM0 with LoRaWAN, ESP-NOW, and cellular backhaul.

### 🧬 AMYNTHAS-HPC (Edaphic & Ecological Modeling)
High-Performance Computing ecosystem model for earthworm dispersal kinetics and edaphic soil carbon flux:
- **Modern Fortran 2018 & C++ Core:** Discretized partial differential transport-reaction equations with AVX-512 SIMD vectorization.
- **Spatial Biogeochemistry:** Coupled continuous soil microclimate metrics with biological population dynamics and PostGIS spatial indexing.

---

## Tech Stack & Core Competencies

| Domain | Technologies & Frameworks |
|---|---|
| **Languages** | Rust (Backend, Wasm, Embedded), C/C++ (Embedded), Modern Fortran (2018), Python, SQL, Bash |
| **Microcontrollers & Architectures** | Espressif (ESP32-P4, ESP32-C6, ESP32-S3), Raspberry Pi (RP2350, RP2040), STMicroelectronics (STM32), TI MSPM0 |
| **Firmware & RTOS** | FreeRTOS, ESP-IDF, Pico SDK, Nanopb (Protocol Buffers), Bare-metal Drivers (I2C, SPI, UART, ADC) |
| **Sensors & Physics** | Optical Dissolved Oxygen (phase fluorometry), Electrochemical pH/ORP, NDIR CO₂, Laser PM, VOC/Gas Resistance |
| **Backend & Cloud** | Rust (Axum, Tokio, Tower), Google Cloud Pub/Sub, WebSockets, JWT (ES256), REST APIs, Docker |
| **Databases & GIS** | PostgreSQL 16, TimescaleDB (Hypertables, compression policies), PostGIS (spatial indexing), SQLx |
| **Frontend & HMI** | Leptos (Rust WebAssembly), Vanilla CSS (Gruvbox Hard Dark design system), Canvas/WebGL telemetry plots |
| **Engineering Quality** | Pedantic Clippy (`-D warnings`), zero unsafe, automated vulnerability audits (`cargo audit`), Conventional Commits |

---

## Connect & Collaborate

- **Website:** [https://oscar-bio.dev/](https://oscar-bio.dev/)
- **Company:** [EcoWest Instrument](https://oscar-bio.dev/)
- **LinkedIn:** [linkedin.com/in/oscar-bio-dev](https://linkedin.com/in/oscar-bio-dev)
- **Email:** [oscar_bio_dev@outlook.com](mailto:oscar_bio_dev@outlook.com)
