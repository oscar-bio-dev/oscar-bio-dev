// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Error devuelto al intentar crear un valor de telemetría con estado inválido.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TelemetryError {
    #[error("pH value {0} is out of valid range (0.0 - 14.0)")]
    Ph(f64),
    #[error("Dissolved oxygen value {0} is out of valid range (0.0 - 40.0 mg/L)")]
    DissolvedOxygen(f64),
    #[error("Temperature value {0} is out of valid range (-50.0 - 150.0 °C)")]
    Temperature(f64),
    #[error("Humidity value {0} is out of valid range (0.0 - 100.0 %RH)")]
    Humidity(f64),
    #[error("Pressure value {0} is out of valid range (300.0 - 1100.0 hPa)")]
    Pressure(f64),
    #[error("Gas Resistance value {0} is out of valid range")]
    GasResistance(f64),
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
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 14.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Ph(value))
        } else {
            Ok(Self(value))
        }
    }
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
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 40.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::DissolvedOxygen(value))
        } else {
            Ok(Self(value))
        }
    }
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
    pub const MIN: f64 = -50.0;
    pub const MAX: f64 = 150.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Temperature(value))
        } else {
            Ok(Self(value))
        }
    }
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
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 100.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Humidity(value))
        } else {
            Ok(Self(value))
        }
    }
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
    pub const MIN: f64 = 300.0;
    pub const MAX: f64 = 1100.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Pressure(value))
        } else {
            Ok(Self(value))
        }
    }
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
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 50_000_000.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::GasResistance(value))
        } else {
            Ok(Self(value))
        }
    }
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
    pub const MIN: f64 = 400.0;
    pub const MAX: f64 = 40_000.0;
    pub fn new(value: f64) -> Result<Self, TelemetryError> {
        if value.is_nan() || !(Self::MIN..=Self::MAX).contains(&value) {
            Err(TelemetryError::Co2(value))
        } else {
            Ok(Self(value))
        }
    }
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

lazy_static::lazy_static! {
    static ref DEVICE_ID_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

/// Payload completo de telemetría proveniente del hardware edge (ESP32, RP2350).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema, Validate)]
pub struct TelemetryPayload {
    /// ID único del dispositivo `IoT`
    #[schema(example = "esp32-node-1")]
    #[validate(
        length(min = 1, max = 64, message = "El device_id debe tener entre 1 y 64 caracteres"),
        regex(path = *DEVICE_ID_REGEX, message = "El device_id solo puede contener letras, números, guiones y guiones bajos")
    )]
    pub device_id: String,

    /// Timestamp de la lectura (UTC)
    #[schema(example = "2026-08-30T10:00:00Z")]
    pub timestamp: DateTime<Utc>,

    /// Temperatura validada en grados Celsius
    pub temperature: Temperature,
    pub humidity: Option<Humidity>,
    pub ph: Option<Ph>,
    pub dissolved_oxygen: Option<DissolvedOxygen>,
    pub pressure: Option<Pressure>,
    pub gas_resistance: Option<GasResistance>,
    pub co2: Option<Co2>,
}

/// DTO binario para Protobuf
#[derive(Clone, PartialEq, prost::Message)]
pub struct TelemetryPayloadPb {
    #[prost(string, tag = "1")]
    pub device_id: String,
    #[prost(int64, tag = "2")]
    pub timestamp_epoch_ms: i64,
    #[prost(double, tag = "3")]
    pub temperature: f64,
    #[prost(double, optional, tag = "4")]
    pub humidity: Option<f64>,
    #[prost(double, optional, tag = "5")]
    pub ph: Option<f64>,
    #[prost(double, optional, tag = "6")]
    pub dissolved_oxygen: Option<f64>,
    #[prost(double, optional, tag = "7")]
    pub pressure: Option<f64>,
    #[prost(double, optional, tag = "8")]
    pub gas_resistance: Option<f64>,
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
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ph_validation(val in -1.0..15.0f64) {
            let ph = Ph::new(val);
            if (0.0..=14.0).contains(&val) {
                assert!(ph.is_ok());
            } else {
                assert!(ph.is_err());
            }
        }

        #[test]
        fn test_temperature_validation(val in -60.0..160.0f64) {
            let temp = Temperature::new(val);
            if (-50.0..=150.0).contains(&val) {
                assert!(temp.is_ok());
            } else {
                assert!(temp.is_err());
            }
        }

        #[test]
        fn test_telemetry_protobuf_conversion(
            temp in -50.0..150.0f64,
            hum in 0.0..100.0f64
        ) {
            let pb = TelemetryPayloadPb {
                device_id: "test-node".to_string(),
                timestamp_epoch_ms: 1700000000000,
                temperature: temp,
                humidity: Some(hum),
                ph: None,
                dissolved_oxygen: None,
                pressure: None,
                gas_resistance: None,
                co2: None,
            };

            let payload: Result<TelemetryPayload, _> = pb.try_into();
            assert!(payload.is_ok());
        }
    }

    #[test]
    fn test_invalid_json_deserialization() {
        let json =
            r#"{"device_id":"node-1","timestamp":"2026-08-30T10:00:00Z","temperature": 999.0}"#;
        let result: Result<TelemetryPayload, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_json_deserialization() {
        let json =
            r#"{"device_id":"node-1","timestamp":"2026-08-30T10:00:00Z","temperature": 25.5}"#;
        let result: Result<TelemetryPayload, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }
}
