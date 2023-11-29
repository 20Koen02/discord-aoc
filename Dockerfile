FROM rust:1 as builder
WORKDIR /usr/src/discord-aoc
COPY Cargo.toml ./
RUN cargo install --path .
COPY . .

FROM debian:12-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/discord-aoc /usr/local/bin/discord-aoc
CMD ["discord-aoc"]
