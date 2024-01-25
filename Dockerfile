FROM node:20 as builder
RUN npm install -g pnpm

WORKDIR /app

COPY ./ ./
WORKDIR /app/frontend

RUN pnpm install
RUN pnpm run build

FROM rust:1.75

WORKDIR /app

COPY ./ ./
# copy pnpm from frontend to app to serve
COPY --from=builder /app/frontend/static/ /app/static/

#expost to port 8000
EXPOSE 8000:8000

# RUN cargo install
RUN cargo build

CMD ["cargo", "run"]
