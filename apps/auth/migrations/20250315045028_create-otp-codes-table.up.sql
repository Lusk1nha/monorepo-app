CREATE TABLE IF NOT EXISTS otp_codes (
  id SERIAL PRIMARY KEY NOT NULL,
  user_id CHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  code VARCHAR(6) NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  used_at TIMESTAMPTZ,
  is_used BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX idx_otp_codes_user_id ON otp_codes(user_id);
DO $$ BEGIN IF NOT EXISTS (
  SELECT 1
  FROM pg_attribute
  WHERE attrelid = 'users'::regclass
    AND attname = 'is_2fa_enabled'
) THEN
ALTER TABLE users
ADD COLUMN is_2fa_enabled BOOLEAN NOT NULL DEFAULT TRUE;
END IF;
IF NOT EXISTS (
  SELECT 1
  FROM pg_attribute
  WHERE attrelid = 'users'::regclass
    AND attname = 'is_email_verified'
) THEN
ALTER TABLE users
ADD COLUMN is_email_verified BOOLEAN NOT NULL DEFAULT FALSE;
END IF;
IF NOT EXISTS (
  SELECT 1
  FROM pg_attribute
  WHERE attrelid = 'users'::regclass
    AND attname = 'otp_secret'
) THEN
ALTER TABLE users
ADD COLUMN otp_secret TEXT;
END IF;
END $$;