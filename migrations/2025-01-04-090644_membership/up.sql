CREATE TABLE membership (
    membership_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,  -- Unique membership ID
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,  -- Reference to the user
    org_id UUID NOT NULL REFERENCES organizations(org_id) ON DELETE CASCADE,  -- Reference to the organization
    role_id UUID NOT NULL REFERENCES roles(role_id) ON DELETE CASCADE,  -- Reference to the user's role within the organization
    metadata JSONB,  -- Additional metadata for the membership like (reason_for_leaving)
    -- status membership_status_enum NOT NULL DEFAULT 'active',  -- Active, Suspended, or other membership statuses
    -- invitation_status membership_invitation_status_enum DEFAULT 'pending',  -- Pending, Accepted, Rejected
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- Timestamp when the user joined the organization
    last_active_at TIMESTAMP WITH TIME ZONE,  -- Last time the user was active within the organization
    expired_at TIMESTAMP WITH TIME ZONE,  -- Optional expiration date for membership (if applicable)
    leave_at TIMESTAMP WITH TIME ZONE,  -- Optional timestamp for when the user leaves the organization
    UNIQUE (user_id, org_id),  -- Ensure one user can only have one active membership in an organization
    -- CONSTRAINT check_membership_status CHECK (status IN ('active', 'suspended', 'inactive')),  -- Validate membership status
    -- CONSTRAINT check_invitation_status CHECK (invitation_status IN ('pending', 'accepted', 'rejected'))  -- Validate invitation status
);
