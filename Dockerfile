# 1. This tells docker to use the Rust official image
FROM rust:1.65

# 2. Copy the files in your machine to the Docker image
COPY ./src /src
COPY ./migrations /migrations
COPY *.toml /

# Build your program for release
RUN cargo build
RUN cargo install diesel_cli --no-default-features --features postgres

# Run the binary
CMD ["diesel migrations run && ./target/debug/acme"]
