-- Add down migration script here
DROP TABLE IF EXISTS credentials;
DROP TRIGGER IF EXISTS trigger_update_timestamp_credentials ON credentials;