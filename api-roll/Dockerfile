FROM rust:1.63

WORKDIR /usr/src/api-roll
COPY . .

RUN cargo install --path .

CMD ["api-roll"]