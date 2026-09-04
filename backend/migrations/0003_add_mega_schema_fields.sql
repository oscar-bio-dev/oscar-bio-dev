-- Migration 0003: Converge backend with the canonical Gateway Mega-Schema.
-- Adds environmental quality (IAQ, PM2.5), particulate matter (BMV080),
-- and node diagnostic fields (battery, sleep cycles).
-- Also relaxes temperature from NOT NULL to nullable for heterogeneous nodes.

-- Air Quality Index (BME688 BSEC output)
ALTER TABLE telemetry ADD COLUMN iaq DOUBLE PRECISION;

-- Particulate Matter (BMV080 sensor)
ALTER TABLE telemetry ADD COLUMN pm1_0 DOUBLE PRECISION;
ALTER TABLE telemetry ADD COLUMN pm2_5 DOUBLE PRECISION;
ALTER TABLE telemetry ADD COLUMN pm10_0 DOUBLE PRECISION;

-- Node Diagnostics
ALTER TABLE telemetry ADD COLUMN battery_mv INTEGER;
ALTER TABLE telemetry ADD COLUMN sleep_cycles INTEGER;

-- Relax temperature: some nodes may only report CO2 or PM data
ALTER TABLE telemetry ALTER COLUMN temperature DROP NOT NULL;
