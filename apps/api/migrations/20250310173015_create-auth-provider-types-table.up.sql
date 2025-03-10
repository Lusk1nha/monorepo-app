-- Criação da tabela `auth_provider_types`
CREATE TABLE IF NOT EXISTS auth_provider_types (
  name VARCHAR(255) PRIMARY KEY NOT NULL,
  description TEXT NULL DEFAULT NULL,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
INSERT INTO auth_provider_types (name, description)
VALUES ('CREDENTIALS', 'Password-based authentication');
INSERT INTO auth_provider_types (name, description)
VALUES ('GOOGLE', 'Google authentication');
INSERT INTO auth_provider_types (name, description)
VALUES ('GITHUB', 'GitHub authentication');