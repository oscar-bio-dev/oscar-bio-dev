// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.

use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use leptos::*;
use shared::TelemetryPayload;

#[component]
pub fn RoomMonitoring() -> impl IntoView {
    let (twin_state, set_twin_state) =
        create_signal(std::collections::HashMap::<String, TelemetryPayload>::new());

    create_effect(move |_| {
        let ws_url = "wss://localhost:3000/api/ws";
        if let Ok(ws) = WebSocket::open(ws_url) {
            let (_, mut read) = ws.split();

            spawn_local(async move {
                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if let Ok(payload) = serde_json::from_str::<TelemetryPayload>(&text) {
                            set_twin_state.update(|state| {
                                state.insert(payload.device_id.clone(), payload);
                            });
                        }
                    }
                }
            });
        }
    });

    view! {
        <div>
            <h2 class="section-title">"Room Monitoring - Esp32 Node"</h2>
            <div class="project-grid" style="grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); display: grid;">
                {move || twin_state.get().values().map(|payload| view! {
                    <section class="terminal-card">
                        <div class="card-header">
                            <span class="dot red" aria-hidden="true"></span>
                            <span class="dot yellow" aria-hidden="true"></span>
                            <span class="dot green" aria-hidden="true"></span>
                            <span class="filename">{payload.device_id.clone()} ".log"</span>
                        </div>
                        <div class="card-body">
                            <p style="color: var(--accent-yellow);">"Temperature: " {format!("{:.1}", payload.temperature.value())} " °C"</p>
                            {payload.humidity.map(|h| view! { <p style="color: var(--accent-blue);">"Humidity: " {format!("{:.1}", h.value())} " %"</p> })}
                            {payload.ph.map(|ph| view! { <p style="color: var(--accent-green);">"pH: " {format!("{:.2}", ph.value())}</p> })}
                            {payload.dissolved_oxygen.map(|do_val| view! { <p style="color: var(--fg-main);">"DO: " {format!("{:.2}", do_val.value())} " mg/L"</p> })}
                            {payload.pressure.map(|p| view! { <p style="color: var(--accent-yellow);">"Pressure: " {format!("{:.1}", p.value())} " hPa"</p> })}
                            {payload.gas_resistance.map(|g| view! { <p style="color: var(--accent-blue);">"Gas Res: " {g.value()} " Ohms"</p> })}
                            {payload.co2.map(|c| view! { <p style="color: var(--accent-red);">"CO2: " {c.value()} " ppm"</p> })}
                        </div>
                    </section>
                }).collect_view()}
            </div>
        </div>
    }
}
