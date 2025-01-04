CREATE TABLE user_roles (
    user_uuid UUID NOT NULL REFERENCES users(user_uuid) ON DELETE CASCADE,
    role_uuid UUID NOT NULL REFERENCES roles(role_uuid) ON DELETE CASCADE,
    assigned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_uuid, role_uuid)
);
