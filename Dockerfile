FROM rust:1.82 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Usar una imagen de Debian con PostgreSQL client
FROM debian:bullseye-slim

# Instalar dependencias necesarias para agregar repositorios
RUN apt-get update && apt-get install -y \
    gnupg \
    lsb-release \
    curl

# Agregar el repositorio de PostgreSQL para instalar la versión correcta
RUN echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release -c | awk '{print $2}')-pgdg main" > /etc/apt/sources.list.d/pgdg.list \
    && curl https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
    && apt-get update

# Instalar PostgreSQL client version 15
RUN apt-get install -y postgresql-client-15 \
    bash \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# Copiar el código desde el contenedor builder
COPY --from=builder /app /app
WORKDIR /app

CMD ["bash"]
