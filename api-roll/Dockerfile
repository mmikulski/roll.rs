FROM rust:1.63 as builder

WORKDIR /usr/src/api-roll
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api-roll /usr/local/bin/api-roll

CMD ["api-roll"]
