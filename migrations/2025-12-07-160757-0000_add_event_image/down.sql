-- This file should undo anything in `up.sql`

-- Remove image_base64 column from events table
ALTER TABLE events
DROP COLUMN IF EXISTS image_base64;

-- Remove image_base64 column from attendees table
ALTER TABLE attendees
DROP COLUMN IF EXISTS image_base64;