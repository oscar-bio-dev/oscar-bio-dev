// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Error devuelto al intentar crear un valor de telemetría con estado inválido.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TelemetryError {
    /// Error en la validación del pH.
    #[error("pH value {0} is out of valid range (0.0 - 14.0)")]
    Ph(f64),
    /// Error en la validación del Oxígeno Disuelto.
    #[error("Dissolved oxygen value {0} is out of valid range (0.0 - 40.0 mg/L)")]
    DissolvedOxygen(f64),
    /// Error en la validación de Temperatura.
    #[error("Temperature value {0} is out of valid range (-50.0 - 150.0 °C)")]
    Temperature(f64),
    /// Error en la validación de Humedad Relativa.
    #[error("Humidity value {0} is out of valid range (0.0 - 100.0 %RH)")]
    Humidity(f64),
    /// Error en la validación de Presión.
    #[error("Pressure value {0} is out of valid range (300.0 - 1100.0 hPa)")]
    Pressure(f64),
    /// Error en la validación de Resistencia de Gas.
    #[error("Gas Resistance value {0} is out of valid range")]
    GasResistance(f64),
    /// Error en la validación de CO2.
    #[error("CO2 value {0} is out of valid range (400.0 - 40000.0 ppm)")]
    Co2(f64),
}

/// Representa el pH del agua o suelo. Garantizado estar entre 0.0 y 14.0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct Ph(f64);

impl TryFrom<f64> for Ph {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Ph {
    /// Valor mínimo físico de pH.
    pub const MIN: f64 = 0.0;
    /// Valor máximo físico de pH.
    pub const MAX: f64 = 14.0;

    /// Crea un nuevo valor de pH. Retorna error si está fuera de rango físico.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Ph(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Representa el Oxígeno Disuelto (mg/L).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct DissolvedOxygen(f64);

impl TryFrom<f64> for DissolvedOxygen {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl DissolvedOxygen {
    /// Valor mínimo físico de Oxígeno Disuelto.
    pub const MIN: f64 = 0.0;
    /// Valor máximo de Oxígeno Disuelto (mg/L).
    pub const MAX: f64 = 40.0; // mg/L, valor máximo físico razonable.

    /// Crea un nuevo valor validado.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::DissolvedOxygen(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Temperatura en grados Celsius.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct Temperature(f64);

impl TryFrom<f64> for Temperature {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Temperature {
    /// Valor mínimo físico de Temperatura.
    pub const MIN: f64 = -50.0;
    /// Valor máximo físico de Temperatura (°C).
    pub const MAX: f64 = 150.0; // Suficiente para aplicaciones industriales ambientales.

    /// Crea un nuevo valor validado de Temperatura.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Temperature(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Humedad Relativa (%).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct Humidity(f64);

impl TryFrom<f64> for Humidity {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Humidity {
    /// Valor mínimo físico de Humedad.
    pub const MIN: f64 = 0.0;
    /// Valor máximo físico de Humedad (%).
    pub const MAX: f64 = 100.0;

    /// Crea un nuevo valor validado de Humedad.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Humidity(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Presión Atmosférica en hPa.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct Pressure(f64);

impl TryFrom<f64> for Pressure {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Pressure {
    /// Valor mínimo físico de Presión.
    pub const MIN: f64 = 300.0;
    /// Valor máximo físico de Presión.
    pub const MAX: f64 = 1100.0;

    /// Crea un nuevo valor validado de Presión.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Pressure(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Resistencia del gas (VOCs) en Ohmios (BME688).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct GasResistance(f64);

impl TryFrom<f64> for GasResistance {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl GasResistance {
    /// Valor mínimo físico de Resistencia.
    pub const MIN: f64 = 0.0;
    /// Valor máximo físico de Resistencia.
    pub const MAX: f64 = 50_000_000.0; // Hasta 50M Ohms

    /// Crea un nuevo valor validado de Resistencia.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::GasResistance(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Nivel de CO2 en ppm (SCD41).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(try_from = "f64")]
pub struct Co2(f64);

impl TryFrom<f64> for Co2 {
    type Error = TelemetryError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Co2 {
    /// Valor mínimo físico de CO2.
    pub const MIN: f64 = 400.0;
    /// Valor máximo físico de CO2.
    pub const MAX: f64 = 40_000.0;

    /// Crea un nuevo valor validado de CO2.
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Co2(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Obtiene el valor flotante crudo validado.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Payload completo de telemetría proveniente del hardware edge (ESP32, RP2350).
#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, validator::Validate,
)]
pub struct TelemetryPayload {
    /// ID único del dispositivo `IoT`
    #[schema(example = "esp32-node-1")]
    #[validate(length(min = 1, message = "El device_id no puede estar vacío"))]
    pub device_id: String,

    /// Timestamp de la lectura (UTC)
    #[schema(example = "2026-08-30T10:00:00Z")]
    pub timestamp: DateTime<Utc>,

    /// Temperatura validada en grados Celsius
    pub temperature: Temperature,
    /// Humedad Relativa (%).
    pub humidity: Option<Humidity>,
    /// Nivel de pH (0.0 - 14.0).
    pub ph: Option<Ph>,
    /// Oxígeno Disuelto (mg/L).
    pub dissolved_oxygen: Option<DissolvedOxygen>,

    /// Presión Atmosférica (hPa).
    pub pressure: Option<Pressure>,
    /// Resistencia de Gas VOCs (Ohms).
    pub gas_resistance: Option<GasResistance>,
    /// CO2 (ppm).
    pub co2: Option<Co2>,
}

/// DTO binario para Protobuf
#[derive(Clone, PartialEq, prost::Message)]
pub struct TelemetryPayloadPb {
    /// ID del dispositivo
    #[prost(string, tag = "1")]
    pub device_id: String,
    /// Timestamp Unix en milisegundos
    #[prost(int64, tag = "2")]
    pub timestamp_epoch_ms: i64,
    /// Temperatura en C
    #[prost(double, tag = "3")]
    pub temperature: f64,
    /// Humedad Relativa
    #[prost(double, optional, tag = "4")]
    pub humidity: Option<f64>,
    /// pH
    #[prost(double, optional, tag = "5")]
    pub ph: Option<f64>,
    /// Oxígeno Disuelto
    #[prost(double, optional, tag = "6")]
    pub dissolved_oxygen: Option<f64>,
    /// Presión en hPa
    #[prost(double, optional, tag = "7")]
    pub pressure: Option<f64>,
    /// Resistencia Gas Ohms
    #[prost(double, optional, tag = "8")]
    pub gas_resistance: Option<f64>,
    /// CO2 ppm
    #[prost(double, optional, tag = "9")]
    pub co2: Option<f64>,
}

impl TryFrom<TelemetryPayloadPb> for TelemetryPayload {
    type Error = TelemetryError;

    fn try_from(pb: TelemetryPayloadPb) -> Result<Self, Self::Error> {
        let timestamp =
            chrono::DateTime::from_timestamp_millis(pb.timestamp_epoch_ms).unwrap_or_default();

        Ok(Self {
            device_id: pb.device_id,
            timestamp,
            temperature: Temperature::new(pb.temperature)?,
            humidity: pb.humidity.map(Humidity::new).transpose()?,
            ph: pb.ph.map(Ph::new).transpose()?,
            dissolved_oxygen: pb.dissolved_oxygen.map(DissolvedOxygen::new).transpose()?,
            pressure: pb.pressure.map(Pressure::new).transpose()?,
            gas_resistance: pb.gas_resistance.map(GasResistance::new).transpose()?,
            co2: pb.co2.map(Co2::new).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ph_valid_bounds() {
        assert!(Ph::new(0.0).is_ok());
        assert!(Ph::new(7.0).is_ok());
        assert!(Ph::new(14.0).is_ok());
    }

    #[test]
    fn test_ph_invalid_bounds() {
        assert_eq!(Ph::new(-0.1), Err(TelemetryError::Ph(-0.1)));
        assert_eq!(Ph::new(14.1), Err(TelemetryError::Ph(14.1)));
        assert!(Ph::new(f64::NAN).is_err());
    }

    #[test]
    fn test_dissolved_oxygen_bounds() {
        assert!(DissolvedOxygen::new(0.0).is_ok());
        assert!(DissolvedOxygen::new(40.0).is_ok());
        assert!(DissolvedOxygen::new(-1.0).is_err());
        assert!(DissolvedOxygen::new(40.1).is_err());
    }

    #[test]
    fn test_temperature_bounds() {
        assert!(Temperature::new(-50.0).is_ok());
        assert!(Temperature::new(150.0).is_ok());
        assert!(Temperature::new(-50.1).is_err());
        assert!(Temperature::new(150.1).is_err());
    }

    #[test]
    fn test_humidity_bounds() {
        assert!(Humidity::new(0.0).is_ok());
        assert!(Humidity::new(100.0).is_ok());
        assert!(Humidity::new(-0.1).is_err());
        assert!(Humidity::new(100.1).is_err());
    }

    #[test]
    fn test_json_deserialization_success() {
        let json_data = r#"{
            "device_id": "PICO_W_01",
            "timestamp": "2026-08-17T12:00:00Z",
            "temperature": 25.4,
            "humidity": 60.2,
            "ph": 7.2,
            "dissolved_oxygen": 8.1
        }"#;

        let payload: Result<TelemetryPayload, _> = serde_json::from_str(json_data);
        assert!(payload.is_ok(), "El JSON válido debería deserializarse correctamente");
    }

    #[test]
    fn test_json_deserialization_failure_out_of_bounds() {
        let json_data = r#"{
            "device_id": "PICO_W_01",
            "timestamp": "2026-08-17T12:00:00Z",
            "temperature": 25.4,
            "ph": 15.0
        }"#;

        let payload: Result<TelemetryPayload, _> = serde_json::from_str(json_data);
        assert!(payload.is_err(), "El JSON con pH 15.0 debería ser rechazado por serde");
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn fuzz_temperature_creation(val in proptest::num::f64::ANY) {
            let _ = Temperature::new(val);
        }

        #[test]
        fn fuzz_humidity_creation(val in proptest::num::f64::ANY) {
            let _ = Humidity::new(val);
        }

        #[test]
        fn fuzz_ph_creation(val in proptest::num::f64::ANY) {
            let _ = Ph::new(val);
        }

        #[test]
        fn fuzz_do_creation(val in proptest::num::f64::ANY) {
            let _ = DissolvedOxygen::new(val);
        }
    }
}
