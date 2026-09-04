-- Migration 0005: Idempotency and DLQ for Pub/Sub Ingestion

-- 1. Modify the telemetry table to support the new Mega-Schema fields
-- Rename timestamp to measured_at
ALTER TABLE telemetry RENAME COLUMN timestamp TO measured_at;
-- Add ingested_at timestamp
ALTER TABLE telemetry ADD COLUMN ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Add routing & sequencing fields
ALTER TABLE telemetry ADD COLUMN protocol_version INTEGER NOT NULL DEFAULT 1;
ALTER TABLE telemetry ADD COLUMN schema_version INTEGER NOT NULL DEFAULT 1;
ALTER TABLE telemetry ADD COLUMN gateway_id VARCHAR(64) NOT NULL DEFAULT 'unknown';
ALTER TABLE telemetry ADD COLUMN node_sequence INTEGER NOT NULL DEFAULT 0;
ALTER TABLE telemetry ADD COLUMN event_id UUID NOT NULL DEFAULT gen_random_uuid();

-- 2. Create a unique constraint for idempotency
-- TimescaleDB hypertables require the partitioning column (measured_at) in the unique index.
CREATE UNIQUE INDEX idx_telemetry_event_id_time ON telemetry (event_id, measured_at);

-- 3. Create the Dead Letter Queue (DLQ) table for Poison Pills
CREATE TABLE telemetry_dlq (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- We store the raw payload as BYTEA since it's a Protobuf binary or JSON string
    raw_payload BYTEA NOT NULL,
    
    -- Error description to understand why it failed validation/domain parsing
    error_reason TEXT NOT NULL,
    
    -- Optional fields if we were able to partially parse them before failure
    gateway_id VARCHAR(64),
    event_id UUID
);

-- Note: We don't make telemetry_dlq a hypertable because it's for exceptional cases 
-- and shouldn't have high volume under normal operation.
