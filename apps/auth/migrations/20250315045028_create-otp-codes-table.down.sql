-- Add down migration script here
DROP TABLE IF EXISTS otp_codes;
ALTER TABLE users DROP COLUMN is_2fa_enabled,
  DROP COLUMN otp_secret, DROP COLUMN is_email_verified;