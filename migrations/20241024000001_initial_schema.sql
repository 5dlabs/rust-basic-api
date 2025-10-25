-- Initial database schema for users table
-- This migration creates the foundational users table with proper indexing and triggers

-- Main users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Performance indexes
-- Index on email for fast lookups and uniqueness enforcement
CREATE INDEX idx_users_email ON users(email);

-- Index on created_at for sorting and filtering by creation date (descending order for recent-first queries)
CREATE INDEX idx_users_created_at ON users(created_at DESC);

-- Trigger function for automatic updated_at updates
-- This function is reusable for any table that needs automatic timestamp updates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to users table
-- This trigger automatically updates updated_at whenever a row is modified
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
