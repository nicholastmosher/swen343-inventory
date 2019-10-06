FROM rust:latest as cargo-build

WORKDIR /inventory

COPY backend/Cargo.toml /inventory/Cargo.toml

RUN mkdir -p src/bin && \
    touch src/lib.rs && \
    echo "fn main() {println!(\"If you see this, the build broke\");}" > src/bin/main.rs && \
    cargo build --release

COPY backend/src /inventory/src

RUN cargo build --release

CMD ["target/release/erp_inventory"]
