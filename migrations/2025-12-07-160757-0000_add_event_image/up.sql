-- Your SQL goes here

-- Add image_base64 column to events table
ALTER TABLE events
ADD COLUMN image_base64 TEXT;

-- Add image_base64 column to users table
ALTER TABLE attendees
ADD COLUMN image_base64 TEXT;
