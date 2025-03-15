CREATE TABLE IF NOT EXISTS auth_providers (
  id BIGSERIAL PRIMARY KEY,
  user_id CHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider VARCHAR(255) NOT NULL,
  provider_user_id VARCHAR(255) NOT NULL UNIQUE,
  access_token TEXT,
  refresh_token TEXT,
  token_expires_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(user_id, provider)
);
CREATE TRIGGER trigger_update_timestamp_auth_providers BEFORE
UPDATE ON auth_providers FOR EACH ROW EXECUTE FUNCTION set_timestamp();