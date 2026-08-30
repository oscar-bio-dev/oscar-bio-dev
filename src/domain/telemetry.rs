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

/// Payload completo de telemetría proveniente del hardware edge (ESP32, RP2350).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TelemetryPayload {
    /// Identificador único del dispositivo.
    pub device_id: String,
    /// Timestamp de lectura en formato ISO 8601 UTC.
    pub timestamp: DateTime<Utc>,
    /// Temperatura en grados Celsius.
    pub temperature: Temperature,
    /// Humedad Relativa (%).
    pub humidity: Option<Humidity>,
    /// Nivel de pH (0.0 - 14.0).
    pub ph: Option<Ph>,
    /// Oxígeno Disuelto (mg/L).
    pub dissolved_oxygen: Option<DissolvedOxygen>,
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
}
