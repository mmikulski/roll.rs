FROM rust:1.64 as builder

RUN apt-get update && apt-get install -y \
    build-essential \
    libtool \
    openssl \
    pkg-config \
    libssl-dev \
    libsasl2-2 \
    libsasl2-dev \
    sed \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/api-roll
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
  openssl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/api-roll /usr/local/bin/api-roll

CMD ["api-roll"]
