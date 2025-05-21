FROM rust:latest as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev libpq-dev clang llvm-dev libclang-dev \
    libzstd-dev liblz4-dev libsnappy-dev libbz2-dev zlib1g-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres --locked

# Setup app
WORKDIR /app
COPY . .

# Set environment variables for bindgen
ENV LIBCLANG_PATH="/usr/lib/llvm-14/lib"
ENV BINDGEN_EXTRA_CLANG_ARGS="-I/usr/lib/llvm-14/include"

# Build the application
RUN cargo build --release

# List the contents of the release directory to verify binary name
RUN ls -la /app/target/release/

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 libssl3 ca-certificates libzstd1 liblz4-1 \
    libsnappy1v5 libbz2-1.0 zlib1g \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary with the correct name (using hyphens as it appears in the build)
COPY --from=builder /app/target/release/sui-indexer-generic /app/

# Copy diesel CLI and migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/
COPY --from=builder /app/migrations /app/migrations/

# Modify the entrypoint script section in the Dockerfile
RUN echo '#!/bin/bash\n\
set -e\n\
\n\
echo "Running migrations..."\n\
diesel setup --database-url="$DATABASE_URL" --migration-dir migrations\n\
diesel migration run --database-url="$DATABASE_URL" --migration-dir migrations\n\
\n\
echo "Starting indexer..."\n\
sleep 10; \n\
if [ "$LOCAL_MODE" = "true" ]; then\n\
  echo "Starting indexer in local mode with dir \"$CHECKPOINT_DIR\"..."\n\
  \n\
  # Check if START_CHECKPOINT is set\n\
  if [ -z "$START_CHECKPOINT" ]; then\n\
    echo "ERROR: START_CHECKPOINT environment variable is not set"\n\
    exit 1\n\
  fi\n\
  \n\
  # Check if the checkpoint file exists\n\
  CHECKPOINT_FILE="$CHECKPOINT_DIR/${START_CHECKPOINT}.chk"\n\
  if [ ! -f "$CHECKPOINT_FILE" ]; then\n\
    echo "ERROR: Checkpoint file $CHECKPOINT_FILE does not exist"\n\
    echo "Available checkpoints in $CHECKPOINT_DIR (showing first 10):"\n\
    ls "$CHECKPOINT_DIR" | grep ".chk" | head -10\n\
    exit 1\n\
  fi\n\
  \n\
  echo "Found checkpoint file: $CHECKPOINT_FILE"\n\
  exec /app/sui-indexer-generic --database-url "$DATABASE_URL" --local-ingestion-path "$CHECKPOINT_DIR" --first-checkpoint "$START_CHECKPOINT" --skip-watermark --package-address "$PACKAGE_ADDRESS"\n\
else\n\
  echo "Starting indexer in remote mode..."\n\
  # Check if PACKAGE_ADDRESS is set\n\
  if [ -z "$PACKAGE_ADDRESS" ]; then\n\
    echo "ERROR: PACKAGE_ADDRESS environment variable is not set"\n\
    exit 1\n\
  fi\n\
  echo "Using package address: $PACKAGE_ADDRESS"\n\
  exec /app/sui-indexer-generic --remote-store-url "$REMOTE_STORE_URL" --database-url "$DATABASE_URL" --first-checkpoint "$START_CHECKPOINT" --skip-watermark --package-address "$PACKAGE_ADDRESS"\n\
fi\n\
' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
