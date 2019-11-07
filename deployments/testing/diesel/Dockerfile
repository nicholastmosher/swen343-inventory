FROM rust:latest

WORKDIR /inventory/backend
COPY ./backend /inventory/backend

RUN apt-get update && apt-get install libpq-dev

RUN cargo install diesel_cli --no-default-features --features "postgres"

# Wait 2 seconds for database to initialize before trying reset
CMD ["bash", "-c", "sleep 2 && diesel database reset"]
