-- Add up migration script here
CREATE TABLE auth_refresh_tokens (
  id CHAR(36) PRIMARY KEY NOT NULL,
  user_id CHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token_hash TEXT NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  revoked_at TIMESTAMPTZ
);
-- Automatically update 'updated_at' on any update
CREATE OR REPLACE FUNCTION set_timestamp() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_update_timestamp_auth_refresh_tokens BEFORE
UPDATE ON auth_refresh_tokens FOR EACH ROW EXECUTE FUNCTION set_timestamp();