FROM rust:latest as cargo-build

WORKDIR /inventory

COPY backend/Cargo.toml /inventory/Cargo.toml

RUN mkdir -p src/bin && \
    touch src/lib.rs && \
    echo "fn main() {println!(\"If you see this, the build broke\");}" > src/bin/main.rs && \
    cargo build --release

COPY backend/src /inventory/src

# If you don't touch these files, Cargo won't see the update and recompile.
RUN touch src/lib.rs && touch src/bin/main.rs

RUN cargo build --release

CMD ["target/release/erp"]
