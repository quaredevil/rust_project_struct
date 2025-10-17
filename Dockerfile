# ---- Build Stage ----
FROM rust:latest AS builder

ARG APP_VERSION=dev
ENV APP_VERSION=${APP_VERSION} \
    CARGO_TERM_COLOR=always \
    CARGO_INCREMENTAL=0 \
    CARGO_NET_RETRY=10 \
    RUSTUP_MAX_RETRIES=10

WORKDIR /app

# Atualiza rustup e instala a versão mais recente
RUN rustup update stable && rustup default stable

# Instala dependências de sistema necessárias para compilar crates
# Inclui cmake e build-essential para rdkafka
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    cmake \
    build-essential \
    libsasl2-dev \
    libzstd-dev \
  && rm -rf /var/lib/apt/lists/*

# Copia manifestos primeiro para aproveitar cache de dependências
COPY Cargo.toml Cargo.lock ./

# Cria arquivos dummy para compilar dependências em cache separado
# Isso é crucial para projetos com lib + bin
RUN mkdir src \
  && echo 'pub fn dummy() {}' > src/lib.rs \
  && echo 'fn main() { println!("dummy"); }' > src/main.rs \
  && cargo build --release \
  && rm -rf src \
  && rm -rf target/release/.fingerprint/listener-* \
  && rm -rf target/release/deps/listener* \
  && rm -rf target/release/deps/liblistener* \
  && rm -f target/release/listener \
  && rm -f target/release/liblistener*

# Agora copia o código real
COPY src ./src
COPY migrations ./migrations

# Build em release mode (já vem otimizado por padrão)
RUN cargo build --release --bin listener --locked \
  && strip target/release/listener \
  && ls -lh target/release/listener

# ---- Runtime Stage ----
FROM debian:trixie-slim AS runtime

ARG APP_VERSION=dev

WORKDIR /app

# Pacotes mínimos de runtime
# Incluindo bibliotecas necessárias para rdkafka e postgres
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    libsasl2-2 \
    libzstd1 \
    curl \
  && rm -rf /var/lib/apt/lists/* \
  && update-ca-certificates

# Cria usuário não-root
RUN groupadd -r appuser -g 1000 \
  && useradd -u 1000 -r -g appuser -m -d /app -s /sbin/nologin appuser

# Copia binário do builder
COPY --from=builder --chown=appuser:appuser /app/target/release/listener /usr/local/bin/listener

# Copia migrations (se necessário em runtime)
COPY --chown=appuser:appuser migrations ./migrations

# Copia entrypoint se existir
COPY --chown=appuser:appuser scripts/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Variáveis de ambiente
ENV APP_PORT=8080 \
    RUST_LOG=info,listener=debug \
    RUST_BACKTRACE=1 \
    APP_VERSION=${APP_VERSION}

EXPOSE 8080

# Metadados OCI
LABEL org.opencontainers.image.title="crypto-listener" \
      org.opencontainers.image.version="${APP_VERSION}" \
      org.opencontainers.image.description="Cryptocurrency market data listener" \
      org.opencontainers.image.licenses="MIT"

# Healthcheck
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -fsS http://localhost:${APP_PORT}/health || exit 1

USER appuser

ENTRYPOINT ["/entrypoint.sh"]
CMD ["listener"]
