-- Criação da tabela `users`
CREATE TABLE IF NOT EXISTS users (
  id VARCHAR(255) PRIMARY KEY NOT NULL,
  -- Identificador único do usuário
  name VARCHAR(255) NULL,
  -- Nome do usuário
  image TEXT NULL,
  -- URL da imagem do usuário
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  -- Data e hora de criação
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  -- Data e hora da última atualização
  last_login_at TIMESTAMPTZ NULL,
  -- Data e hora do último login
  is_active BOOLEAN DEFAULT TRUE NOT NULL -- Status do usuário (ativo/inativo)
);
-- Comentários para documentação
COMMENT ON TABLE users IS 'Tabela que armazena informações dos usuários.';
COMMENT ON COLUMN users.id IS 'Identificador único do usuário (CHAR(36)).';
COMMENT ON COLUMN users.name IS 'Nome do usuário.';
COMMENT ON COLUMN users.image IS 'URL da imagem do usuário.';
COMMENT ON COLUMN users.created_at IS 'Data e hora de criação do registro.';
COMMENT ON COLUMN users.updated_at IS 'Data e hora da última atualização do registro.';
COMMENT ON COLUMN users.last_login_at IS 'Data e hora do último login do usuário.';
COMMENT ON COLUMN users.is_active IS 'Indica se o usuário está ativo (TRUE) ou inativo (FALSE).';
-- Função para atualizar o campo `updated_at`
CREATE OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = CURRENT_TIMESTAMP;
-- Atualiza o campo `updated_at` para o timestamp atual
RETURN NEW;
-- Retorna o novo valor da linha
END;
$$ LANGUAGE plpgsql;
-- Gatilho para atualizar `updated_at` automaticamente
CREATE TRIGGER update_users_updated_at BEFORE
UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
-- Comentário sobre o gatilho
COMMENT ON TRIGGER update_users_updated_at ON users IS 'Gatilho para atualizar automaticamente o campo `updated_at` antes de qualquer operação de UPDATE.';