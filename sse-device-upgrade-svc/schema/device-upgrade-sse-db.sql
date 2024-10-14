CREATE TABLE device_upgrades (
    id SERIAL PRIMARY KEY,
    device_id TEXT NOT NULL,
    device_firmware_version TEXT DEFAULT '0.0.1',
    device_firmware_kernel TEXT DEFAULT 'arduino_linux',
    firmware_upgrade_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    firmware_next_upgrade_date TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '1 day',
    contains_patch BOOLEAN DEFAULT false
);
