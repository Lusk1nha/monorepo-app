-- Add down migration script here
DROP TABLE IF EXISTS auth_refresh_tokens;
DROP TRIGGER IF EXISTS trigger_update_timestamp_auth_refresh_tokens ON auth_refresh_tokens;