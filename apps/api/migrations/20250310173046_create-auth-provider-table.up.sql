-- Criação da tabela `auth_provider`
CREATE TABLE IF NOT EXISTS auth_providers (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  -- Identificador único do provedor de autenticação
  provider_type VARCHAR(255) NOT NULL,
  -- Tipo do provedor de autenticação (credentials, google, github)
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  -- Data e hora de criação
  FOREIGN KEY (id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
  -- Chave estrangeira para a tabela `users`
  FOREIGN KEY (provider_type) REFERENCES auth_provider_types (name) ON DELETE CASCADE ON UPDATE CASCADE,
  -- Chave estrangeira para a tabela `auth_types`
  UNIQUE (id, provider_type) -- Garante que um usuário não tenha mais de um provedor do mesmo tipo
);
-- Comentários para documentação
COMMENT ON TABLE auth_providers IS 'Tabela que armazena os provedores de autenticação associados aos usuários.';
COMMENT ON COLUMN auth_providers.id IS 'Identificador único do provedor de autenticação (CHAR(36)).';
COMMENT ON COLUMN auth_providers.provider_type IS 'Tipo do provedor de autenticação (credentials, google, github).';
COMMENT ON COLUMN auth_providers.created_at IS 'Data e hora de criação do registro.';
-- Índice para consultas por user_id
CREATE INDEX idx_auth_providers_provider_type ON auth_providers (provider_type);
-- Índice para consultas por tipo de provedor
CREATE INDEX idx_auth_providers_user_id_provider_type ON auth_providers (id, provider_type);