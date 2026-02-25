# ── Stage 1: Build ──────────────────────────────────────────────────────────
FROM rust:1.83-slim AS builder

# Install system deps needed by some crates (e.g. openssl, pkg-config)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies separately from source for faster rebuilds.
# Copy only manifest files first and build a dummy main so Cargo caches deps.
COPY Cargo.toml ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo generate-lockfile
RUN cargo build --release
RUN rm -rf src

# Now copy real source and build
COPY src ./src
# Touch main.rs so Cargo knows it changed
RUN touch src/main.rs
RUN cargo build --release

# ── Stage 2: Development image with watcher installed ──────────────────────────
FROM rust:1.83-slim AS dev

# Install system deps that may be needed for building and installing tools
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates libssl3 build-essential git inotify-tools && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="$CARGO_HOME/bin:$PATH"

RUN mkdir -p $CARGO_HOME $RUSTUP_HOME $CARGO_HOME/bin

WORKDIR /app

# We prefer a simple inotify-based loop for dev inside the container to avoid relying on installing newer Rust toolchain crates
# (previous attempts to cargo-install watchexec/cargo-watch hit toolchain/version issues).
# Compose will mount the host sources over /app so cargo run will use the host files.

EXPOSE 8080
# Default command runs an inotify loop that restarts cargo run on Rust source changes
CMD ["sh", "-lc", "while true; do /usr/local/cargo/bin/cargo run & pid=$$!; inotifywait -e modify -r src || true; kill \"$$pid\" 2>/dev/null || true; wait \"$$pid\" 2>/dev/null || true; sleep 0.2; done"]

# ── Stage 3: Minimal runtime ─────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backend .

EXPOSE 8080
CMD ["./backend"]
