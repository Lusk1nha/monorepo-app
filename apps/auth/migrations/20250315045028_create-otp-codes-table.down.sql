-- Add down migration script here
DROP TABLE IF EXISTS otp_codes;
ALTER TABLE users DROP COLUMN otp_secret;