-- MailNow Database Setup Script
-- Run this script to create the database and user

-- Create database
CREATE DATABASE mailnow_db;

-- Create user
CREATE USER mailnow_user WITH PASSWORD 'mailnow_pass';

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE mailnow_db TO mailnow_user;

-- Connect to the database
\c mailnow_db;

-- Grant schema privileges
GRANT ALL ON SCHEMA public TO mailnow_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO mailnow_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO mailnow_user;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO mailnow_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO mailnow_user;