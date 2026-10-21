# syntax=docker/dockerfile:1

# ---------------------------------------------------------------------------
# Build stage
# ---------------------------------------------------------------------------
FROM rust:1-bookworm AS builder

WORKDIR /app

# Cache dependency compilation: build a dummy crate first so that a source
# change does not force a full rebuild of every dependency.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && echo '' > src/lib.rs \
    && cargo build --release \
    && rm -rf src

# Build the real application.
COPY . .
RUN touch src/main.rs src/lib.rs && cargo build --release

# ---------------------------------------------------------------------------
# Runtime stage
# ---------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 app

WORKDIR /app

COPY --from=builder /app/target/release/ferristak /usr/local/bin/ferristak
# Migrations are read at runtime by the `migrate` command / startup hook.
COPY --from=builder /app/migrations ./migrations

USER app

ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/ferristak"]
CMD ["serve"]
