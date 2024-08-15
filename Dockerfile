FROM rust AS prep
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

FROM prep AS builder
WORKDIR /usr/src/app
COPY . .
RUN trunk build --release

FROM nginx:alpine AS runtime
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
