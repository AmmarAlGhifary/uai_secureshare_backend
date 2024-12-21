-- Add migration script here
-- Ensure the UUID extension is availables for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users Table
CREATE Table users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nama VARCHAR(100) NOT NULL,
    password VARCHAR(255) NOT NULL, -- Disimpan secara hashed
    public_key TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Files table
CREATE Table files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    users_id UUID REFERENCES users(id) ON DELETE CASCADE, -- Foreign key untuk table users
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    encrypted_aes_key BYTEA NOT NULL,  -- Store Encrypted AES key
    encrypted_file BYTEA NOT NULL, -- Store the actual encrypted file content
    iv BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE shared_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_id UUID REFERENCES files(id) ON DELETE CASCADE, -- Missing comma fixed here
    receipiet_user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- Fixed column type and missing comma
    password VARCHAR(100) NOT NULL,
    expiration_date TIMESTAMP WITH TIME ZONE NOT NULL, -- Fixed "WITH ZONE" to "WITH TIME ZONE"
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
