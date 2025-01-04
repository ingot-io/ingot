-- Your SQL goes here
CREATE TABLE permissions (
    permission_uuid UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    permission_key VARCHAR(255) UNIQUE NOT NULL,      -- e.g., "can_create_project", "can_edit_profile"
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
