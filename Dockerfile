FROM rust:1.72 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    postgresql-client bash && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app /app
WORKDIR /app
CMD ["bash"]