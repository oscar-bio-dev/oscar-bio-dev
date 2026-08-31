-- Add Air Quality monitoring fields (BME688 / SCD41)
ALTER TABLE telemetry ADD COLUMN pressure DOUBLE PRECISION;
ALTER TABLE telemetry ADD COLUMN gas_resistance DOUBLE PRECISION;
ALTER TABLE telemetry ADD COLUMN co2 DOUBLE PRECISION;
