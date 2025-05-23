# Etapa 1: Construção do aplicativo
FROM rust:latest AS builder

# Definir o diretório de trabalho dentro do contêiner
WORKDIR /app

# Copiar o .env para dentro do container
COPY .env .env

# Definir variáveis de ambiente no contêiner
ARG ENVIRONMENT
ENV ENVIRONMENT=production

# Copiar o arquivo Cargo.toml e Cargo.lock primeiro (para cache)
COPY Cargo.toml ./ 

# # Baixar dependências (isso evita refazer o download se não houver mudanças no Cargo.toml)
# RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch

# Copiar o restante dos arquivos do projeto, incluindo pastas como migrations
COPY . .  

# Compilar o aplicativo em modo release
RUN cargo build --release

# Etapa 2: Imagem final mínima
FROM debian:bookworm-slim

# Instalar bibliotecas necessárias para o binário Rust
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && \
  apt-get clean && rm -rf /var/lib/apt/lists/*

# Definir o diretório de trabalho
WORKDIR /app

# Copiar o binário da etapa de construção
COPY --from=builder /app/target/release/auth ./auth

# Copiar as pastas adicionais, como migrations, para a imagem final
COPY --from=builder /app/apps/auth/migrations ./migrations 

# Expor a porta do servidor (ajuste conforme necessário)
EXPOSE 8080

# Comando de inicialização
CMD ["./auth"]


