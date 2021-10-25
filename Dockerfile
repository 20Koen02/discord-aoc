FROM rust:1.56 as builder
WORKDIR /usr/src/discord-aoc
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/discord-aoc /usr/local/bin/discord-aoc
CMD ["discord-aoc"]