FROM rust:latest

WORKDIR /inventory

COPY backend/Cargo.toml /inventory/Cargo.toml

RUN cargo install systemfd cargo-watch

RUN mkdir -p src/bin && \
    touch src/lib.rs && \
    echo "fn main() {println!(\"If you see this, the build broke\");}" > src/bin/main.rs && \
    cargo build && \
    rm -f target/debug/deps/erp_inventory*

CMD systemfd --no-pid -s http::8000 -- cargo watch -x run
