FROM rust:1.84-slim-bookworm AS backend-builder
WORKDIR /usr/app
COPY ./src ./src/

COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build \
    -r \
    --target-dir=/usr

FROM node:current-alpine3.20 AS frontend-builder
WORKDIR /usr/app

COPY ./package.json .
COPY ./package-lock.json .
COPY ./assets ./assets

RUN npm install

FROM debian:bookworm-slim
WORKDIR /usr/app

COPY --from=backend-builder /usr/app .
COPY --from=backend-builder /usr/release/rust-htop ./rust-htop
COPY --from=frontend-builder /usr/app/node_modules ./node_modules
COPY --from=frontend-builder /usr/app/assets ./assets

CMD ["/usr/app/rust-htop"]
