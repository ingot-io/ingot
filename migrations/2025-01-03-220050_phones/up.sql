CREATE TABLE phones (
    phone_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,  -- Unique identifier for the phone record
    user_uuid UUID NOT NULL REFERENCES users(user_uuid) ON DELETE CASCADE,  -- Foreign key to users table
    country_code VARCHAR(5) NOT NULL,  -- Country code (e.g., "+1", "+44")
    number VARCHAR(20) NOT NULL,  -- Phone number without the country code
    full_number VARCHAR(30) GENERATED ALWAYS AS (country_code || number) STORED UNIQUE,  -- Combined full phone number (unique)

    type VARCHAR(20) DEFAULT 'mobile',  -- Type of phone (e.g., mobile, work, home, etc.)
    status phone_status_enum NOT NULL DEFAULT 'unverified',  -- Phone number status
    
    verified_at TIMESTAMP(3),  -- Timestamp of when email was verified
    bounced_at TIMESTAMP(3),  -- Timestamp of when email was bounced

    is_primary BOOLEAN NOT NULL DEFAULT FALSE,  -- Flag to indicate whether this is the user's primary email
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,  -- A boolean flag for faster verification checks

    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    
    CONSTRAINT phone_unique_per_user UNIQUE (user_uuid, full_number),  -- Ensure unique phone numbers per user
);

-- Create a unique index on the email value
CREATE UNIQUE INDEX phones_value_idx ON phones(full_number);

-- Create a unique index on the user_uuid and is_primary columns
CREATE UNIQUE INDEX phones_user_uuid_is_primary_idx ON phones(user_uuid, is_primary);

