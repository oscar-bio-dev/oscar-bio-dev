# Oscar Mora | Tecnólogo de Conservación e Ingeniero de Sistemas Embebidos

[![Portafolio](https://img.shields.io/badge/Sitio_Web-oscar--bio.dev-2a9d8f?style=for-the-badge&logo=googlechrome&logoColor=white)](https://oscar-bio.dev/)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-oscar--bio--dev-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white)](https://linkedin.com/in/oscar-bio-dev)
[![GitHub](https://img.shields.io/badge/GitHub-oscar--bio--dev-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/oscar-bio-dev)
[![Rust](https://img.shields.io/badge/Rust-1.85+-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![C/C++](https://img.shields.io/badge/C%2FC%2B%2B-Embebido-00599C?style=for-the-badge&logo=c%2B%2B&logoColor=white)](https://isocpp.org/)
[![Licencia: MIT](https://img.shields.io/badge/Licencia-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

[English](README.md) • [Versión en Español](README.es.md)

---

## Sobre Mí

Soy **Tecnólogo de Conservación**, **Biólogo Marino** e **Ingeniero de Sistemas Embebidos** radicado en Pueblo West, Colorado. Uno la investigación ecológica con la instrumentación bare-metal, traduciendo requisitos ambientales complejos en nodos IoT de ultrabajo consumo, infraestructura de telemetría resiliente y plataformas analíticas de alta velocidad.

Como fundador de **[EcoWest Instrument](https://oscar-bio.dev/)**, diseño y fabrico sistemas de hardware de grado industrial, etapas AFE de acondicionamiento de señal y software de telemetría concebidos para operar en condiciones de campo extremas.

- **Sitio Web y Portafolio:** [https://oscar-bio.dev/](https://oscar-bio.dev/)
- **Especialización:** Monitoreo ambiental in-situ, telemetría edge-to-cloud, arquitecturas distribuidas de series de tiempo, firmware bare-metal y modelado computacional de biodiversidad edáfica.
- **Ubicación:** Pueblo West, Colorado, Estados Unidos.

---

## Proyectos Destacados e Iniciativas

### 📡 [Ecological Telemetry Hub](https://github.com/oscar-bio-dev/telemetry-hub)
Plataforma edge-to-cloud de ingestión de telemetría y Gemelo Digital (*Digital Twin*) desarrollada en Rust (`Axum`, `Tokio`) y WebAssembly (`Leptos`), respaldada por TimescaleDB y PostGIS.
- **Mega-Schema Unificado (33 campos):** Contrato Protobuf con seguridad de tipos compartido entre firmware C (`nanopb`) y microservicios Rust (`prost`).
- **Ingestión Resiliente y DLQ:** Suscriptor Pull asíncrono de GCP Pub/Sub con enrutamiento automático de paquetes corruptos hacia Dead-Letter Queue.
- **Gemelo Digital Cognitivo:** Caché de estado en memoria (`Arc<RwLock<LruCache>>`) con razonamiento contextual de IA.
- **HMI Wasm de Alto Rendimiento:** Panel industrial en Leptos Wasm con estética Gruvbox Hard Dark, diagnósticos de calibración de sensores en vivo y sincronización por WebSocket sin sobrecarga de JavaScript.

### 🔬 [EcoWest Instrument](https://oscar-bio.dev/)
Instrumentación científica y comercial para ecosistemas de suelo y acuáticos:
- **Oxígeno Disuelto Óptico (WQM-V100):** Medición óptica de DO de alta precisión por tiempo de decaimiento luminiscente sin deriva inducida por CPU.
- **Nodos Ambientales Multiparamétricos:** Nodos sensores de bajo consumo integrando Bosch Sensortec BME688 (IAQ, VOC), Sensirion SCD41 (CO₂ NDIR) y Bosch BMV080 (material particulado láser PM1.0/PM2.5/PM10).
- **Gateways Industriales:** Gateways de telemetría basados en ESP32-P4, RP2350 y MSPM0 con conectividad LoRaWAN, ESP-NOW y enlace celular.

### 🧬 AMYNTHAS-HPC (Modelado Ecológico y Edáfico)
Motor de Computación de Alto Rendimiento (HPC) para la cinética de dispersión de lombrices de tierra y flujos de carbono en el suelo:
- **Núcleo en Fortran 2018 y C++:** Ecuaciones diferenciales parciales de transporte-reacción discretizadas con vectorización SIMD AVX-512.
- **Biogeoquímica Espacial:** Acoplamiento de métricas continuas de microclima edáfico con dinámicas biológicas poblacionales e indexación espacial en PostGIS.

---

## Stack Tecnológico y Competencias Principales

| Dominio | Tecnologías y Herramientas |
|---|---|
| **Lenguajes** | Rust (Backend, Wasm, Embebido), C/C++ (Embebido), Fortran Moderno (2018), Python, SQL, Bash |
| **Microcontroladores y Arquitecturas** | Espressif (ESP32-P4, ESP32-C6, ESP32-S3), Raspberry Pi (RP2350, RP2040), STMicroelectronics (STM32), TI MSPM0 |
| **Firmware y RTOS** | FreeRTOS, ESP-IDF, Pico SDK, Nanopb (Protocol Buffers), Drivers bare-metal (I2C, SPI, UART, ADC) |
| **Sensores y Física** | Oxígeno Disuelto Óptico (fluorometría de fase), pH/ORP electroquímico, CO₂ NDIR, PM láser, resistencia de gases/VOC |
| **Backend y Nube** | Rust (Axum, Tokio, Tower), Google Cloud Pub/Sub, WebSockets, JWT (ES256), APIs REST, Docker |
| **Bases de Datos y GIS** | PostgreSQL 16, TimescaleDB (Hypertables, políticas de compresión), PostGIS (indexación espacial), SQLx |
| **Frontend y HMI** | Leptos (Rust WebAssembly), CSS nativo (sistema de diseño Gruvbox Hard Dark), gráficos de telemetría en Canvas/WebGL |
| **Calidad de Ingeniería** | Clippy pedante (`-D warnings`), cero unsafe, auditorías automatizadas de seguridad (`cargo audit`), Conventional Commits |

---

## Contacto y Redes

- **Sitio Web Personal:** [https://oscar-bio.dev/](https://oscar-bio.dev/)
- **Empresa:** [EcoWest Instrument](https://oscar-bio.dev/)
- **LinkedIn:** [linkedin.com/in/oscar-bio-dev](https://linkedin.com/in/oscar-bio-dev)
- **Correo Electrónico:** [oscar_bio_dev@outlook.com](mailto:oscar_bio_dev@outlook.com)
