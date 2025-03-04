-- Add down migration script here
ALTER TABLE posts
DROP COLUMN deleted_at;
