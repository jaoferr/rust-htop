FROM rust:1.84-slim-bookworm AS backend-builder
WORKDIR /usr/app
COPY ./src ./src/

COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build \
    -r \
    --target=x86_64-unknown-linux-musl \
    --target-dir=/usr

FROM node:current-alpine3.20 AS frontend-builder
WORKDIR /usr/app

COPY ./package.json .
COPY ./package-lock.json .
COPY ./assets ./assets

RUN npm install

FROM alpine:3.21
WORKDIR /usr/app

COPY --from=backend-builder /usr/app .
COPY --from=backend-builder /usr/x86_64-unknown-linux-musl/release/rust-htop ./rust-htop
COPY --from=frontend-builder /usr/app/node_modules ./node_modules
COPY --from=frontend-builder /usr/app/assets ./assets

CMD ["/usr/app/rust-htop"]
