CREATE TABLE sessions (
    session_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,  -- Unique session ID
    user_uuid UUID NOT NULL REFERENCES users(user_uuid) ON DELETE CASCADE,  -- Reference to the user
    device_uuid UUID,  -- Reference to the device used for the session (if applicable)
    ip_address INET NOT NULL,  -- User's IP address (supports IPv6)
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,  -- Additional session-related data (e.g., custom user data, preferences)
    last_accessed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- Last access time for the session
    is_active BOOLEAN NOT NULL DEFAULT TRUE,  -- Indicates whether the session is active
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,  -- Expiration time for the session
    invalidated_at TIMESTAMP WITH TIME ZONE,  -- Timestamp when the session was invalidated (if applicable)
    revoked_at TIMESTAMP WITH TIME ZONE,  -- Timestamp when the session was revoked (if applicable)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  -- Timestamp when session was created
);

-- Indexes
CREATE INDEX idx_sessions_user_uuid ON sessions(user_uuid);
CREATE INDEX idx_sessions_device_uuid ON sessions(device_uuid);
CREATE INDEX idx_sessions_ip_address ON sessions(ip_address);