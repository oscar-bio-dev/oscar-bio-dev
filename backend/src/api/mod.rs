// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.
//
// This file is part of the oscar-bio-dev platform.
// Unauthorized copying of this file, via any medium, is strictly prohibited.
// Proprietary and confidential.

/// Submódulo encargado del Chatbot LLM impulsado por Gemini.
pub mod chat;
/// Submódulo encargado de la lectura global del gemelo digital.
pub mod digital_twin;
/// Submódulo encargado de la ingesta de diagnósticos del Edge Gateway.
pub mod gateway_health;
/// Submódulo de Kubernetes Probes
pub mod health;
/// Submódulo encargado de la recepción y validación HTTP de la telemetría.
pub mod telemetry;
/// Submódulo encargado del streaming y control en tiempo real.
pub mod ws;
