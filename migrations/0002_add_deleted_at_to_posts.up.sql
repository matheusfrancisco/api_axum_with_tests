-- Add up migration script here
ALTER TABLE posts
ADD COLUMN deleted_at TIMESTAMP;
