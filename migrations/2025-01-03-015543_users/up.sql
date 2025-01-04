-- Create the users table
CREATE TABLE users (
    user_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash BYTEA NOT NULL,  -- Store Argon2 password hash as raw bytes (256 bytes)
    status user_status_enum NOT NULL DEFAULT 'active', -- active, inactive, banned, deleted, etc.
    is_verified BOOLEAN NOT NULL  DEFAULT FALSE,
    onboarded BOOLEAN NOT NULL DEFAULT FALSE,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    archived_at TIMESTAMP(3) DEFAULT NULL,
    created_at TIMESTAMP(3) NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL  DEFAULT CURRENT_TIMESTAMP
);