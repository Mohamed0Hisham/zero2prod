# Builder Stage
FROM lukemathwalker/cargo-chef:latest-rust-1.83.0 as chef
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends lld clang \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release --bin zero2prod

# Runtime Stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Run as non-root user for security
RUN useradd --no-create-home --shell /bin/false appuser
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
RUN chown -R appuser:appuser /app
USER appuser

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./zero2prod"]