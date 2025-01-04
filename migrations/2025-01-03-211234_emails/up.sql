CREATE TABLE emails (
    email_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_uuid UUID NOT NULL REFERENCES users(user_uuid) ON DELETE CASCADE,
    value VARCHAR(255) NOT NULL UNIQUE,  -- The actual email address (localpart@domain)
    status email_status_enum NOT NULL DEFAULT 'unverified',
    
    verified_at TIMESTAMP(3),  -- Timestamp of when email was verified
    bounced_at TIMESTAMP(3),  -- Timestamp of when email was bounced

    is_primary BOOLEAN NOT NULL DEFAULT FALSE,  -- Flag to indicate whether this is the user's primary email
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,  -- A boolean flag for faster verification checks

    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create a unique index on the email value
CREATE UNIQUE INDEX emails_value_idx ON emails(value);

-- Create a unique index on the user_uuid and is_primary columns
CREATE UNIQUE INDEX emails_user_uuid_is_primary_idx ON emails(user_uuid, is_primary);