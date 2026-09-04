-- Migration 0004: Gateway Health Events table (Contract-First).
-- Stores diagnostic events from the Edge Telemetry Gateway (ESP32-P4).
-- Published independently of sensor telemetry via GCP Pub/Sub topic `gateway_health`.

CREATE TABLE gateway_health_events (
    id UUID DEFAULT gen_random_uuid(),
    gateway_id VARCHAR(64) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,

    -- Status
    is_degraded_mode BOOLEAN NOT NULL DEFAULT FALSE,

    -- MicroSD / Storage Diagnostics
    sd_card_mounted BOOLEAN NOT NULL DEFAULT TRUE,
    sd_card_total_mb INTEGER,
    sd_card_free_mb INTEGER,
    sd_io_errors INTEGER NOT NULL DEFAULT 0,

    -- Uptime & System
    uptime_seconds INTEGER NOT NULL DEFAULT 0,
    free_heap_bytes INTEGER NOT NULL DEFAULT 0,

    -- Alert
    alert_message TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, timestamp)
);

SELECT create_hypertable('gateway_health_events', 'timestamp');

CREATE INDEX idx_gw_health_gateway_time ON gateway_health_events (gateway_id, timestamp DESC);
