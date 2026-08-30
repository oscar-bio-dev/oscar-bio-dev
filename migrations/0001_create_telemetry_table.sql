CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE telemetry (
    id UUID DEFAULT gen_random_uuid(),
    device_id VARCHAR(128) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    temperature DOUBLE PRECISION NOT NULL,
    humidity DOUBLE PRECISION,
    ph DOUBLE PRECISION,
    dissolved_oxygen DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, timestamp)
);

SELECT create_hypertable('telemetry', 'timestamp');

CREATE INDEX idx_telemetry_device_time ON telemetry (device_id, timestamp DESC);
