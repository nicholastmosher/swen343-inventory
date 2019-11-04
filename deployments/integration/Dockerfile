FROM rust:latest

WORKDIR /inventory/backend
COPY ./backend /inventory/backend

RUN apt-get update && apt-get install libpq-dev

RUN cargo install diesel_cli --no-default-features --features "postgres"

CMD ["diesel", "setup"]
