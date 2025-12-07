-- This file should undo anything in `up.sql`

ALTER TABLE attendees
DROP COLUMN IF EXISTS password_hash;
ALTER TABLE attendees
DROP COLUMN IF EXISTS salt;
ALTER TABLE attendees
DROP COLUMN IF EXISTS is_admin;
