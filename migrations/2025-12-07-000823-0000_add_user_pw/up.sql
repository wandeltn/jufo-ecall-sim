-- Your SQL goes here

-- Store hashed passwords, can be NULL for guest users
ALTER TABLE attendees
ADD COLUMN password_hash VARCHAR(255); 
-- Store salt for password hashing, can be NULL for guest users
ALTER TABLE attendees
ADD COLUMN salt VARCHAR(255); 
-- Flag to indicate if the user has admin privileges
ALTER TABLE attendees
ADD COLUMN is_admin BOOLEAN DEFAULT FALSE; 
