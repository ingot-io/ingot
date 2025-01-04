CREATE TABLE roles (
    role_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    role_name VARCHAR(100) UNIQUE NOT NULL,        -- e.g., "admin", "member"
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
