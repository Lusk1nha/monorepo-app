-- Criação da tabela `credentials`
CREATE TABLE IF NOT EXISTS credentials (
  id CHAR(36) PRIMARY KEY NOT NULL,
  -- Identificador único da credencial
  user_id CHAR(36) NOT NULL,
  -- Identificador único do usuário
  email VARCHAR(255) UNIQUE NOT NULL,
  -- Endereço de e-mail único
  password_hash TEXT NOT NULL,
  -- Hash da senha (flexível para diferentes algoritmos)
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  -- Data e hora de criação
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  -- Data e hora da última atualização
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE -- Chave estrangeira para a tabela `users`
);
COMMENT ON TABLE credentials IS 'Tabela que armazena as credenciais de autenticação dos usuários.';
COMMENT ON COLUMN credentials.id IS 'Identificador único da credencial (CHAR(36)).';
COMMENT ON COLUMN credentials.user_id IS 'Identificador único do usuário associado à credencial (CHAR(36)).';
COMMENT ON COLUMN credentials.email IS 'Endereço de e-mail único associado à credencial.';
COMMENT ON COLUMN credentials.password_hash IS 'Hash da senha do usuário, gerado por um algoritmo de criptografia.';
COMMENT ON COLUMN credentials.created_at IS 'Data e hora de criação da credencial.';
COMMENT ON COLUMN credentials.updated_at IS 'Data e hora da última atualização da credencial.';
CREATE INDEX idx_credentials_email ON credentials (email);
CREATE OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER update_credentials_updated_at BEFORE
UPDATE ON credentials FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
COMMENT ON TRIGGER update_credentials_updated_at ON credentials IS 'Gatilho para atualizar automaticamente o campo `updated_at` antes de qualquer operação de UPDATE na tabela `credentials`.';