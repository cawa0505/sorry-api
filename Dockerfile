# Build stage
FROM rust:1.98-slim AS build
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# Build with a dummy main to warm the cache the lazy way isn't possible here;
# just build it once.
RUN cargo build --release

# Runtime stage — the joke fits in 10 MB.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /build/target/release/sorry-api /usr/local/bin/sorry-api
EXPOSE 8080
ENV SORRY_PORT=8080
CMD ["sorry-api"]