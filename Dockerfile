FROM node:lts AS frontend

WORKDIR /inventory/frontend
COPY ./frontend .
RUN yarn
RUN yarn build

FROM rust:latest as backend
WORKDIR /inventory/backend
COPY ./backend .
RUN cargo build

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
