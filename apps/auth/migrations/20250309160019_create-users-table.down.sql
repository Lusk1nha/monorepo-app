-- Add down migration script here
DROP TABLE IF EXISTS users;
DROP TRIGGER IF EXISTS trigger_update_timestamp_users ON users;
DROP FUNCTION IF EXISTS set_timestamp;