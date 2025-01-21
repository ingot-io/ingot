CREATE TABLE role_permissions (
    role_uuid UUID NOT NULL REFERENCES roles(role_uuid) ON DELETE CASCADE,
    permission_uuid UUID NOT NULL REFERENCES permissions(permission_uuid) ON DELETE CASCADE,
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_uuid, permission_uuid)
);
