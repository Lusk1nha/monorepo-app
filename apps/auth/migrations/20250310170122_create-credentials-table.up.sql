CREATE TABLE IF NOT EXISTS credentials (
  id CHAR(36) PRIMARY KEY NOT NULL,
  user_id CHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  password_hash TEXT NOT NULL,
  algorithm VARCHAR(20) NOT NULL CHECK (
    algorithm IN ('bcrypt', 'argon2', 'pbkdf2', 'scrypt')
  ),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(user_id)
);
CREATE TRIGGER trigger_update_timestamp_credentials BEFORE
UPDATE ON credentials FOR EACH ROW EXECUTE FUNCTION set_timestamp();