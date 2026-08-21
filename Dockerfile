# =============================================================================
# Stage 1: builder
# Compila o binário em modo release usando a imagem oficial do Rust.
# Separar o build do runtime reduz drasticamente o tamanho da imagem final.
# =============================================================================
FROM rust:1.96 AS builder

# A imagem rust:1.96 (não-slim) já inclui: git, curl, build-essential, gcc.
# Adicionamos apenas as ferramentas extras exigidas pelo boring-sys v5.x,
# que compila o BoringSSL do zero como parte do build do jwt-simple:
#
#   - cmake       : sistema de build do BoringSSL
#   - golang-go   : o cmake do BoringSSL chama `go` internamente
#   - ninja-build : gerador CMake preferido pelo boring-sys
#   - libclang-dev: exigido pelo bindgen para gerar bindings C→Rust do BoringSSL
#   - pkg-config  : resolução de bibliotecas nativas pelo cargo
#   - libssl-dev  : cabeçalhos OpenSSL para sqlx e similares
#
# O tamanho desta imagem NÃO afeta a imagem final de produção,
# pois o stage runtime copia apenas o binário compilado.
RUN apt-get update && apt-get install -y \
    cmake \
    golang-go \
    ninja-build \
    pkg-config \
    libssl-dev \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# SQLX_OFFLINE=true: instrui o sqlx a usar o cache .sqlx/ gerado por
# `cargo sqlx prepare` em vez de conectar ao banco em tempo de compilação.
# Isso é obrigatório para builds offline (Docker, CI/CD).
# Pré-requisito: executar `cargo sqlx prepare` localmente antes do build.
ENV SQLX_OFFLINE=true

# --- Cache de dependências (camada separada para acelerar rebuilds) ---
# Copia apenas os manifestos primeiro. Assim, se só o src/ mudar,
# o `cargo build` de dependências não é reexecutado.
COPY Cargo.toml Cargo.lock ./

# Cria um main.rs dummy para forçar o cargo a compilar as dependências
# sem precisar do código-fonte real.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# --- Build do código-fonte real ---
# O diretório .sqlx/ contém os metadados gerados por `cargo sqlx prepare`.
# É necessário para que o macro sqlx::query! compile em modo offline.
# O diretório templates/ é necessário em compile-time pois o Askama é um
# proc-macro que lê e valida os templates durante a compilação do Rust.
COPY .sqlx ./.sqlx
COPY templates ./templates
COPY migrations ./migrations
COPY src ./src
# Toca os arquivos para invalidar o cache do cargo corretamente.
RUN touch src/main.rs
RUN cargo build --release

# =============================================================================
# Stage 2: runtime
# Imagem final mínima: apenas o binário + assets necessários em tempo de execução.
# debian:bookworm-slim é preferível ao scratch pois contém libc e certificados TLS.
# =============================================================================
FROM debian:bookworm-slim AS runtime

# Instala certificados TLS (necessário para sqlx/reqwest em conexões seguras)
# e libssl para linkagem dinâmica.
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia o binário compilado do stage builder.
COPY --from=builder /app/target/release/wallet ./wallet

# Copia assets necessários em tempo de execução:
# - templates/: renderização de views (Askama lê do filesystem em dev mode)
# - migrations/: usadas pelo sqlx::migrate! em runtime
COPY templates ./templates
COPY migrations ./migrations

# Porta exposta pelo servidor Axum (TcpListener::bind("0.0.0.0:3000") em app.rs).
EXPOSE 3000

# Executa o binário diretamente (sem shell) para que SIGTERM seja recebido
# corretamente pelo processo Rust (graceful shutdown).
CMD ["./wallet"]
