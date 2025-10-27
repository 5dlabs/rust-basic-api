-- 001_initial_schema.sql
-- Initial schema for users table with indexes and trigger for updated_at

-- Create extension for UUID if needed in future (safe if exists)
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index to speed up queries by email (unique constraint above already enforces uniqueness)
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);

-- Index to speed up time-ordered queries
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users (created_at);

-- Function to update updated_at column on row updates
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically set updated_at on UPDATE
DROP TRIGGER IF EXISTS trg_set_updated_at ON users;
CREATE TRIGGER trg_set_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();

