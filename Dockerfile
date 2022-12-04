# 1. This tells docker to use the Rust official image
FROM rust:1.65

# 2. Copy the files in your machine to the Docker image
COPY ./src /src
COPY ./migrations /migrations
COPY *.toml /

RUN apt-get update && apt-get dist-upgrade -y \
    && apt-get install gnupg2 wget -y \
    && sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' \
    && wget -qO- https://www.postgresql.org/media/keys/ACCC4CF8.asc | tee /etc/apt/trusted.gpg.d/pgdg.asc &>/dev/null \
    && apt-get update -y \
    && apt-get install libpq-dev -y

# Build your program for release
RUN cargo build
RUN cargo install diesel_cli --no-default-features --features postgres

# Run the binary
CMD ["diesel migrations run && ./target/debug/acme"]
