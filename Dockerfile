FROM rust:1.60-alpine as builder
ENV RUSTFLAGS='-C link-arg=-s'

RUN apk add --no-cache musl-dev

# Build app
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --target x86_64-unknown-linux-musl --release && \
    cp /app/target/x86_64-unknown-linux-musl/release/sunrise-api /app/sunrise-api
RUN strip /app/sunrise-api


# Create app image
FROM scratch
COPY --from=builder /app/sunrise-api /sunrise-api
CMD ["/sunrise-api"]