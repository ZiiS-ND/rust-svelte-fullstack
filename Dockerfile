# syntax=docker/dockerfile:1.4
FROM node:20 as builder

WORKDIR /app

RUN cd ./frontend

COPY ./ ./

RUN npm install -g pnpm
RUN pnpm install
CMD pnpm run build

RUN cd ..

FROM lukemathwalker/cargo-chef:latest-rust-1.75.0 AS chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build

FROM rust:1.75-slim AS rust_api
COPY --from=builder /app/target/release/rust_api /usr/local/bin
# ENTRYPOINT ["/usr/local/bin/template-rust"]