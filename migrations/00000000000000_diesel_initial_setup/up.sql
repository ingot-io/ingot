-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enums
-- Create ENUM type just if it doesn't exist
CREATE TYPE user_status_enum AS ENUM (
    'active',
    'inactive',
    'banned',
    'deleted'
);


CREATE TYPE email_status_enum AS ENUM (
    'verified',
    'unverified',
    'bounced',
    'spam',
    'blocked'
);


CREATE TYPE phone_status_enum AS ENUM (
    'verified',
    'unverified',
    'bounced',
    'spam',
    'blocked'
);

-- Create the ENUM type for device_type
CREATE TYPE device_type_enum AS ENUM (
    'desktop',       -- Standard desktop computers
    'laptop',        -- Laptops
    'mobile',        -- Mobile phones
    'tablet',        -- Tablets
    'iot',           -- Internet of Things devices
    'wearable',      -- Wearable devices like smartwatches
    'gaming_console',-- Gaming consoles like Xbox, PlayStation
    'smart_tv',      -- Smart TVs
    'car',           -- Smart car systems
    'vr_headset',    -- Virtual reality headsets
    'e_reader',      -- E-readers like Kindle
    'server',        -- Servers accessing the app
    'other'          -- Any other devices not explicitly listed
);


-- Create ENUM type for operating systems
CREATE TYPE os_enum AS ENUM (
    'windows',        -- Microsoft Windows
    'macos',          -- Apple macOS
    'linux',          -- Linux distributions
    'android',        -- Google Android
    'ios',            -- Apple iOS
    'chrome_os',      -- Google Chrome OS
    'tizen',          -- Tizen OS (used in Samsung devices, TVs, etc.)
    'fire_os',        -- Fire OS (Amazon devices)
    'watchos',        -- Apple Watch OS
    'webos',          -- LG WebOS (used in smart TVs)
    'freebsd',        -- FreeBSD (less common server/desktop OS)
    'other'           -- Any unrecognized or unspecified OS
);

CREATE TYPE device_status_enum AS ENUM (
    'active',        -- Device is currently active and in use
    'inactive',      -- Device is not in use but still registered
    'suspended',     -- Device is temporarily suspended (e.g., due to security concerns)
    'banned',        -- Device is banned and no longer allowed to access the system
    'pending',       -- Device is pending approval or activation
    'retired',       -- Device is retired and no longer in use
    'lost',          -- Device is reported as lost
    'stolen',        -- Device is reported as stolen
    'maintenance',   -- Device is under maintenance
    'offline'        -- Device is offline (e.g., not connected to the network)
);

CREATE TYPE membership_status_enum AS ENUM (
    'active',        -- Membership is active and the user has full access
    'inactive',      -- Membership is inactive (e.g., user is no longer participating)
    'suspended',     -- Membership is temporarily suspended (e.g., due to policy violations)
    'pending',       -- Membership is pending approval or activation
    'expired',       -- Membership has expired (e.g., after a set duration)
    'revoked',       -- Membership has been revoked by the organization
    'left',          -- User has voluntarily left the organization
    'banned'         -- User is banned from the organization
);

CREATE TYPE membership_invitation_status_enum AS ENUM (
    'pending',       -- Invitation has been sent but not yet accepted or rejected
    'accepted',      -- Invitation has been accepted by the user
    'rejected',      -- Invitation has been rejected by the user
    'expired',       -- Invitation has expired (e.g., after a set time period)
    'revoked'        -- Invitation has been revoked by the organization
);

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
