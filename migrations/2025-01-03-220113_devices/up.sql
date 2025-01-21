CREATE TABLE devices (
    device_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,  -- Unique identifier for the device record
    device_type device_type_enum NOT NULL,  -- Type of device (e.g., desktop, mobile, tablet, IoT)
    os os_enum NOT NULL,  -- Operating system (e.g., Windows, macOS, Android, iOS)
    os_version VARCHAR(50),  -- OS version (e.g., 11.0, 10.15.7)
    browser VARCHAR(100),  -- Browser used (e.g., Chrome, Firefox, Safari)
    browser_version VARCHAR(50),  -- Browser version
    ip_address INET NOT NULL,  -- IP address of the device
    location JSONB,  -- JSON field to store location details (e.g., country, city, latitude, longitude)
    status device_status_enum NOT NULL DEFAULT 'active',  -- Device status (active, inactive, etc.)
    last_used_at TIMESTAMP WITH TIME ZONE,  -- Timestamp when the device was last used
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- When the record was created
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- When the record was last updated
    mac_address VARCHAR(17), -- MAC address of the device (for network-related tracking)
    notification_token TEXT,                  -- Token for push notifications
    notification_enabled BOOLEAN DEFAULT TRUE, -- Whether notifications are enabled
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB  -- Additional metadata stored as JSON
);

CREATE INDEX idx_devices_device_type ON devices(device_type);
CREATE INDEX idx_devices_os ON devices(os);
