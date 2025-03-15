-- Add down migration script here
DROP TABLE IF EXISTS auth_providers;
DROP TRIGGER IF EXISTS trigger_update_timestamp_auth_providers ON auth_providers;