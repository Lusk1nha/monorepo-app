-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  last_login_at TIMESTAMPTZ NULL,
  is_active BOOLEAN DEFAULT TRUE NOT NULL
);

-- Criação de uma função para atualizar o campo `updated_at`
CREATE
OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $ $ BEGIN NEW.updated_at = CURRENT_TIMESTAMP;

RETURN NEW;

END;

$ $ LANGUAGE plpgsql;

-- Criação do gatilho para atualizar `updated_at` automaticamente
CREATE TRIGGER update_users_updated_at BEFORE
UPDATE
  ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();