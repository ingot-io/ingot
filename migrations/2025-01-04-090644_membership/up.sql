CREATE TABLE membership (
    membership_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,  -- Unique membership ID
    user_uuid UUID NOT NULL REFERENCES users(user_uuid) ON DELETE CASCADE,  -- Reference to the user
    org_uuid UUID NOT NULL REFERENCES organizations(org_uuid) ON DELETE CASCADE,  -- Reference to the organization
    role_uuid UUID NOT NULL REFERENCES roles(role_uuid) ON DELETE CASCADE,  -- Reference to the user's role within the organization
    metadata JSONB,  -- Additional metadata for the membership like (reason_for_leaving)
    status membership_status_enum NOT NULL DEFAULT 'active',  -- Active, Suspended, or other membership statuses
    invitation_status membership_invitation_status_enum DEFAULT 'pending',  -- Pending, Accepted, Rejected
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- Timestamp when the user joined the organization
    last_active_at TIMESTAMP WITH TIME ZONE,  -- Last time the user was active within the organization
    expired_at TIMESTAMP WITH TIME ZONE,  -- Optional expiration date for membership (if applicable)
    leave_at TIMESTAMP WITH TIME ZONE,  -- Optional timestamp for when the user leaves the organization
    UNIQUE (user_uuid, org_uuid)  -- Ensure one user can only have one active membership in an organization
);
