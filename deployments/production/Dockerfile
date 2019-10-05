FROM rust:latest as cargo-build

WORKDIR /inventory

COPY backend/ /inventory/

RUN cargo build --release

CMD ["target/release/erp_inventory"]
