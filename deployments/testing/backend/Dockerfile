FROM rust:latest as backend

RUN apt-get update && apt-get install libpq-dev
WORKDIR /inventory/backend

# Build backend dependencies
COPY ./backend/Cargo.toml .
RUN mkdir -p src/bin && \
    touch src/lib.rs && \
    echo "fn main() {println!(\"If you see this, the build broke\");}" > src/bin/main.rs && \
    cargo build

# Build and test backend
COPY ./backend/src ./src
RUN touch src/bin/main.rs && touch src/lib.rs && cargo test --all && cargo build

FROM postman/newman:ubuntu
RUN apt-get update && apt-get install -y libpq-dev
WORKDIR /inventory
COPY --from=backend /inventory/backend/target/debug/erp ./backend/erp
COPY ./backend/postman ./postman

ENV BIND_ADDRESS 0.0.0.0:8000
ENV RUST_LOG="actix_web=INFO,erp=DEBUG"
ENTRYPOINT []
CMD ["bash", "-c", "(sleep 2 && backend/erp) & (sleep 3 && newman run postman/Inventory-Full-V1.postman_collection.json -e postman/Development.postman_environment.json)"]

EXPOSE 8000
