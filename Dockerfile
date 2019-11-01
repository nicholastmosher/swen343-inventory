FROM rust:latest as backend
WORKDIR /inventory/backend
COPY ./backend/Cargo.toml .
RUN mkdir -p src/bin && \
    touch src/lib.rs && \
    echo "fn main() {println!(\"If you see this, the build broke\");}" > src/bin/main.rs && \
    cargo build
COPY ./backend/src ./src
RUN touch src/bin/main.rs && touch src/lib.rs && cargo build

FROM node:lts AS frontend
WORKDIR /inventory/frontend
COPY ./frontend .
RUN yarn
RUN yarn build

FROM centos:latest
RUN yum install -y libpq
WORKDIR /inventory
RUN mkdir -p frontend/build
COPY --from=frontend /inventory/frontend/build ./frontend/build
COPY --from=backend /inventory/backend/target/debug/erp ./backend/erp

ENV BIND_ADDRESS http://0.0.0.0:8000
ENV FRONTEND_PUBLIC_PATH /inventory/frontend/build
CMD ["backend/erp"]

EXPOSE 8000
