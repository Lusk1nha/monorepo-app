CREATE TABLE IF NOT EXISTS email_logs (
  id SERIAL PRIMARY KEY,
  category VARCHAR(255) NOT NULL,
  status VARCHAR(255) NOT NULL,
  from_address TEXT NOT NULL,
  to_address TEXT,
  cc_address TEXT,
  bcc_address TEXT,
  subject TEXT NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);