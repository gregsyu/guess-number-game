FROM rust:1-slim-bookworm

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["target/release/guess_number"]