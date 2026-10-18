# Build
FROM rust as build

WORKDIR /discord-cat
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./apps ./apps
COPY ./libs ./libs
RUN cargo build --release --bin discord-cat

# Deploy to a slim image
FROM debian:12-slim

RUN apt-get update
RUN apt-get install ca-certificates -y

COPY --from=build /discord-cat/target/release/discord-cat .
CMD ["./discord-cat"]
